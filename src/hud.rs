use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};

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
