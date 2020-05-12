use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use super::gamestate::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

impl GameState<'_, '_> {
    pub fn enable_console(&mut self){
        self.vidsub.text_input().start();
        self.console = Some(Console::new());
    }
    pub fn disable_console(&mut self){
        self.vidsub.text_input().stop();
        self.console = None;
    }
}

pub struct Console {
    hist: Vec<String>,
    outp: Vec<String>,
    pub inp: String,
    //size and font??
}

impl Console {
    pub fn new() -> Console {
        Console {
            hist: vec!(),
            outp: vec!(),
            inp: String::new(),
        }
    }
    pub fn render_console(&self, canv: &mut WindowCanvas /*, font: Font*/) -> bool {
        let texture_creator = canv.texture_creator();
        let mut surf = Surface::new(500, 300, PixelFormatEnum::RGB24).unwrap();
        let text = texture_creator.create_texture_from_surface(&mut surf).unwrap();
        match canv.copy_ex(&text, None, Rect::new(0, 0, 500, 300), 0.0, None, false, false) {
            Ok(_f) => {},
            Err(_e) => {eprintln!("error in rendering player");},
        }
        true
    }
    pub fn update_console(&self) -> bool {
        
        true
    }
}
