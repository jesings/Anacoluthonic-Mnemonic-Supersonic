use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::ttf::Font;

struct Console {
    hist: Vec<str>,
    outp: Vec<str>,
    //size and font??
}

//hijack/restore keyboard controls
impl Console {
    fn render_console(&self, canv: WindowCanvas, font: Font) -> bool {
        true
    }
    fn update_console(&self, canv: WindowCanvas, font: Font) -> bool {
        true
    }
}
