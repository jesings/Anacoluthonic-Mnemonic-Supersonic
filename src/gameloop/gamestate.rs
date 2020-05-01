use sdl2::render::{WindowCanvas};
use super::grid::*;
use sdl2::pixels::Color;
use sdl2::rect::*;

pub struct GameState<'a> {
    pub canvas: &'a mut WindowCanvas,
    pub grid: &'a mut Grid,
}

impl GameState<'_> {
    pub fn clear(&mut self){
        self.canvas.clear()
    }
    pub fn present(&mut self){
        self.canvas.present()
    }
    pub fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b))
    }
    pub fn draw_point(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.canvas.draw_point(Point::new(x, y))
    }
}
