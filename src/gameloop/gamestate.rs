use sdl2::render::{WindowCanvas};
use super::grid::Grid;
use super::entities::{Player, Entity};
use sdl2::pixels::Color;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;

static ACCEL: f64 = 1.0 / 64.0;

pub struct GameState {
    pub canvas: WindowCanvas,
    pub grid: Grid,
    pub player: Player,
    pub pump: sdl2::EventPump,
}

impl GameState {
    pub fn clear(&mut self) {
        self.canvas.clear()
    }
    pub fn present(&mut self) {
        self.canvas.present()
    }
    pub fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b))
    }
    pub fn update(&mut self) -> bool {
        let kbs =  self.pump.keyboard_state();
        let left = kbs.is_scancode_pressed(Scancode::A) || kbs.is_scancode_pressed(Scancode::Left);
        let down = kbs.is_scancode_pressed(Scancode::S) || kbs.is_scancode_pressed(Scancode::Down);
        let right = kbs.is_scancode_pressed(Scancode::D) || kbs.is_scancode_pressed(Scancode::Right);
        let up = kbs.is_scancode_pressed(Scancode::W) || kbs.is_scancode_pressed(Scancode::Up);
        let cw = kbs.is_scancode_pressed(Scancode::E);
        let ccw = kbs.is_scancode_pressed(Scancode::Q);

        let mut ddxdt: f64 = 0.0;
        let mut ddydt: f64 = 0.0;
        if left { ddxdt -= ACCEL; }
        if right { ddxdt += ACCEL; }
        if up { ddydt -= ACCEL; }
        if down { ddydt += ACCEL; }
        let mut rv: f64 = 0.0;
        if cw { rv += 0.9; }
        if ccw { rv -= 0.9; }

        let nrmlzr = ddxdt.hypot(ddydt);
        ddxdt *= nrmlzr;
        ddydt *= nrmlzr;

        self.player.change_vel(ddxdt, ddydt);
        self.player.rotate(rv);

        //loop over all entities, for now we just do player
        self.player.apply_vel(&self.grid);
        //propagate changes to the server as well here
        true
    }
}

