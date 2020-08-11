use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::render::{WindowCanvas};

use std::collections::HashMap;
use std::sync::{Arc};

use super::gamestate::*;
use super::client::connect;

pub trait MenuRender {
    fn render(&self, canv: &mut WindowCanvas, fontmap: &mut HashMap<String, Font>, xdim: i32, ydim: i32) -> bool;
}

pub struct Button {
    pub height: f32,
    pub width: f32,
    pub cx: f32,
    pub cy: f32,
    pub text: String,
    //texture
    pub font: String,
    pub textcolor: Color,
    pub bgcolor: Color,
    pub callback: fn(&mut GameState) -> bool,
}

pub struct Slider {
    width: f32,
    cx: f32,
    cy: f32,
    //nubtexture
    //linetexture
    lineheight: f32,
    nubpos: f32,
    nubdims: f32,
}

pub fn gotogame(gs: &mut GameState) -> bool {
  println!("Pushed start game button");
  gs.scene = Scenes::GamePlay(GameplayScene::None);
    // to be called when connecting from menu
  connect(Arc::clone(&gs.gamedata), &gs.address);
  true
}

pub fn fdummy(_gs: &mut GameState) -> bool {
  println!("Pushed dummy button");
  true
}

impl MenuRender for Button {
    fn render(&self, canv: &mut WindowCanvas, fontmap: &mut HashMap<String, Font>, xdim: i32, ydim: i32) -> bool {
        let iwidth = (self.width * xdim as f32) as i32;
        let iheight = (self.height * ydim as f32) as i32;
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - iwidth / 2;
        let corny = icy - iheight / 2;
        let wrecked = Rect::new(cornx, corny, iwidth as u32, iheight as u32);
        canv.set_draw_color(self.bgcolor);
        match canv.fill_rect(wrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering button background, {}", e);
                return false;
            },
        }
        let fontguy = match fontmap.get(&self.font) {
            Some(g) => g,
            None => {
                eprintln!("Error rendering specified font {}", self.font);
                return false;
            },
        };
        let partial = fontguy.render(self.text.as_str());
        let mut textsurf = match partial.blended(self.textcolor) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error rendering text on button, {}", e);
                return false;
            },
        };

        let tratio = textsurf.width() as f32 / textsurf.height() as f32;
        let bratio = iwidth as f32 / iheight as f32;
        let newwidth = if tratio < bratio {(iheight as f32 * tratio) as u32} else {iwidth as u32};
        let newheight = if tratio < bratio {iheight as u32} else {(iwidth as f32 / bratio) as u32};

        let cornx2 = cornx + ((iwidth - newwidth as i32) / 2) as i32;
        let corny2 = corny + ((iheight - newheight as i32) / 2) as i32;

        let texture_creator = canv.texture_creator();
        let text = texture_creator.create_texture_from_surface(&mut textsurf).unwrap();
        match canv.copy(&text, None, Rect::new(cornx2, corny2, newwidth, newheight)) {
            Ok(_f) => {},
            Err(_e) => {eprintln!("error in rendering button");},
        }
        
        true
    }
}
impl MenuRender for Slider {
    fn render(&self, canv: &mut WindowCanvas, _fontmap: &mut HashMap<String, Font>, xdim: i32, ydim: i32) -> bool {
        let iwidth = (self.width * xdim as f32) as i32;
        let iheight = (self.lineheight * ydim as f32) as i32;
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - iwidth / 2;
        let corny = icy - iheight / 2;
        let wrecked = Rect::new(cornx, corny, iwidth as u32, iheight as u32);
        canv.set_draw_color(Color::RGB(230, 60, 60));
        match canv.draw_rect(wrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering slider background, {}", e);
                return false;
            },
        }

        let nubcorn = (self.nubdims * ydim as f32) as i32;
        let nubx = (self.nubpos * self.width * xdim as f32) as i32 + icx;
        let cornubx = nubx - nubcorn / 2;
        let cornuby = icy - nubcorn / 2;
        let nubwrecked = Rect::new(cornubx, cornuby, nubcorn as u32, nubcorn as u32);
        canv.set_draw_color(Color::RGB(12, 250, 100));
        match canv.draw_rect(nubwrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering slider background, {}", e);
                return false;
            },
        }

        //println!("{} {}", twidth, theight);

        //let texture_creator = canv.texture_creator();
        //let text = texture_creator.create_texture_from_surface(&mut textsurf).unwrap();
        //match self.canvas.copy_ex(&text, None, Rect::new(, topy, xlen, ylen), gdata.player.rot(), ppt, false, false) {
        
        true
    }
}
