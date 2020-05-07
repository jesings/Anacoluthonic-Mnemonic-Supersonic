use sdl2::render::{WindowCanvas};
use super::grid::Grid;
use super::entities::{Player, Entity};
use sdl2::pixels::Color;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;

static ACCEL: f64 = 1.0 / 64.0;
pub struct GameData {
    pub grid: Grid,
    //&entities????
    pub player: Player,
}
pub struct MenuItems {
    name: String,
    //buttons: 
    //sliders: 
    //??? the above need function callbacks, not sure about click and drag for sliders
}
pub enum Scenes {
    Menu(MenuItems),
    GamePlay(GameData),
    //No Clue what to put here
}

pub struct GameState {
    pub canvas: WindowCanvas,
    pub pump: sdl2::EventPump,
    //pub entities: &dyn T, where T is Entity
    pub scene: Scenes,
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

        //get what keycodes symbolize, we can use client keyboard settings to do that
        //After, serialize and send over

        match &mut self.scene {
            Scenes::GamePlay(gdata) => {
                let gpv = gdata.player.vel();
                let rot = gdata.player.rot();

                if up { 
                    let ddxdt: f64 = rot.to_radians().cos() * ACCEL;
                    let ddydt: f64 = rot.to_radians().sin() * ACCEL;

                    gdata.player.change_vel(ddxdt, ddydt);
                }

                if down { 
                    let dir = gpv.y.atan2(gpv.x);
                    let mut ddxdt: f64 = -dir.cos() * 1.5 * ACCEL;
                    let mut ddydt: f64 = -dir.sin() * 1.5 * ACCEL;

                    //if the signs are not equal and back would cause direction change, set velocity to 0 instead
                    if gpv.x.signum() != ddxdt.signum() && ddxdt.abs() > gpv.x.abs() {ddxdt = -gpv.x;}
                    if gpv.y.signum() != ddydt.signum() && ddydt.abs() > gpv.y.abs() {ddydt = -gpv.y;}
                    gdata.player.change_vel(ddxdt, ddydt);
                }

                let mut rv: f64 = 0.0;
                if right { rv += 3.0; }
                if left { rv -= 3.0; }
                gdata.player.rotate(rv);

                //loop over all entities, for now we just do player
                gdata.player.apply_vel(&gdata.grid);
                //propagate changes to the server as well here
            },
            Scenes::Menu(t) => {},
        }
        true
    }
}

