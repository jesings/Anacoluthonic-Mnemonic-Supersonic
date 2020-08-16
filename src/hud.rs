use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::gameloop::gamestate::{GameData};

pub struct HudItem {
    pub width: i32,
    pub height: i32,
    pub xpadding: i32,
    pub ypadding: i32,
    pub bgcolor: Color,
}
impl HudItem {
    pub fn render(&self, canv: &mut WindowCanvas, xdim: i32, ydim: i32) -> bool {
        let cornx = if self.xpadding > 0 {self.xpadding} else {xdim + self.xpadding - self.width};
        let corny = if self.ypadding > 0 {self.ypadding} else {ydim + self.ypadding - self.height};
        let wrecked = Rect::new(cornx, corny, self.width as u32, self.height as u32);
        canv.set_draw_color(self.bgcolor);
        match canv.fill_rect(wrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering hud item background, {}", e);
                return false;
            },
        }
        true
    }
}

pub struct HudText {
    pub width: i32,
    pub height: i32,
    pub xpadding: i32,
    pub ypadding: i32,
    pub font: String,
    pub textgen: fn(&GameData) -> String,
}
impl HudText {
    pub fn render(&self, gd: &GameData, canv: &mut WindowCanvas, fonthash: &HashMap<String, Font>, xdim: i32, ydim: i32) -> bool {
        let cornx = if self.xpadding > 0 {self.xpadding} else {xdim + self.xpadding - self.width};
        let corny = if self.ypadding > 0 {self.ypadding} else {ydim + self.ypadding - self.height};

        let text = (self.textgen)(gd);

        let fontguy = match fonthash.get(&self.font) {
            Some(g) => g,
            None => {
                eprintln!("Error rendering specified font {}", self.font);
                return false;
            },
        };

        let partial = fontguy.render(text.as_str());
        let mut textsurf = match partial.blended(Color::RGB(255, 255, 255)) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error rendering text on button, {}", e);
                return false;
            },
        };


        let texture_creator = canv.texture_creator();
        let text = texture_creator.create_texture_from_surface(&mut textsurf).unwrap();
        let surfheight = textsurf.height() as i32;
        let surfwidth = textsurf.width() as i32;
        let cornx2 = cornx - if self.width < surfwidth {((surfwidth - self.width) / 2)} else {0};
        let corny2 = corny - if self.height < surfheight {((surfheight - self.height) / 2)} else {0};
        match canv.copy(&text, None, Rect::new(cornx2, corny2, surfwidth as u32, surfheight as u32)) {
            Ok(_f) => {},
            Err(_e) => {eprintln!("error in rendering button");},
        }
        
        true
    }
}
