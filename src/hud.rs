use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};

pub struct HudItem {
    pub dims: i32,
    pub cx: f32,
    pub cy: f32,
    pub bgcolor: Color,
}
impl HudItem {
    pub fn render(&self, canv: &mut WindowCanvas, xdim: i32, ydim: i32) -> bool {
        let icx = (self.cx * xdim as f32) as i32;
        let icy = (self.cy * ydim as f32) as i32;
        let cornx = icx - self.dims / 2;
        let corny = icy - self.dims / 2;
        let wrecked = Rect::new(cornx, corny, self.dims as u32, self.dims as u32);
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
