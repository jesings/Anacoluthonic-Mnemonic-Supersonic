use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf::Font;

use super::gamestate::*;

trait MenuRender {
    fn render(&self, gs: &mut GameState, xdim: i32, ydim: i32) -> bool;
}

pub struct Button {
    height: f32,
    width: f32,
    cx: f32,
    cy: f32,
    text: String,
    //texture
    font: String,
    textcolor: Color,
}

pub struct Slider {
    width: f32,
    cx: f32,
    cy: f32,
    //nubtexture
    //linetexture
    nubheight: f32,
    lineheight: f32,
    nubpos: f32,
    nubdims: f32,
}

impl MenuRender for Button {
    fn render(&self, gs: &mut GameState, xdim: i32, ydim: i32) -> bool {
        let iwidth = (self.width * xdim as f32) as i32;
        let iheight = (self.height * ydim as f32) as i32;
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - iwidth / 2;
        let corny = icy - iheight / 2;
        let wrecked = Rect::new(cornx, corny, iwidth as u32, iheight as u32);
        gs.canvas.set_draw_color(Color::RGB(0, 80, 160));
        match gs.canvas.draw_rect(wrecked) {
            Ok(_g) => {},
            Err(e) => {
                eprintln!("Error rendering button background, {}", e);
                return false;
            },
        }
        let fontguy = match gs.fonts.get(&self.font) {
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

        let twidth = textsurf.width();
        let theight = textsurf.height();

        println!("{} {}", twidth, theight);

        //let texture_creator = canv.texture_creator();
        //let text = texture_creator.create_texture_from_surface(&mut textsurf).unwrap();
        //match self.canvas.copy_ex(&text, None, Rect::new(, topy, xlen, ylen), gdata.player.rot(), ppt, false, false) {
        
        true
    }
}
impl MenuRender for Slider {
    fn render(&self, gs: &mut GameState, xdim: i32, ydim: i32) -> bool {
        let iwidth = (self.width * xdim as f32) as i32;
        let iheight = (self.lineheight * ydim as f32) as i32;
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - iwidth / 2;
        let corny = icy - iheight / 2;
        let wrecked = Rect::new(cornx, corny, iwidth as u32, iheight as u32);
        gs.canvas.set_draw_color(Color::RGB(230, 60, 60));
        match gs.canvas.draw_rect(wrecked) {
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
        gs.canvas.set_draw_color(Color::RGB(12, 250, 100));
        match gs.canvas.draw_rect(nubwrecked) {
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
