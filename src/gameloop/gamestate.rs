use sdl2::render::{WindowCanvas};
use sdl2::pixels::Color;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::*;
use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::ttf::Font;
use std::collections::HashMap;

use super::grid::Grid;
use super::entities::{Player, Entity};
use super::console::*;
use super::menu::{Button, Slider};

static ACCEL: f64 = 1.0 / 64.0;
pub struct GameData {
    pub grid: Grid,
    //&entities????
    pub player: Player,
}
pub struct MenuItems {
    pub name: String,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    //??? the above need function callbacks, not sure about click and drag for sliders
}
pub enum Scenes {
    Menu(MenuItems),
    GamePlay(GameData),
    //No Clue what to put here
}

pub struct GameState<'ttf, 'a> {
    pub canvas: WindowCanvas,
    pub pump: sdl2::EventPump,
    pub console: Option<Console>,
    pub fonts: HashMap<String, Font<'ttf, 'a>>,
    //pub entities: &dyn T, where T is Entity
    pub vidsub: VideoSubsystem,
    pub scene: Scenes,
}

impl GameState<'_, '_> {
    pub fn update(&mut self) -> bool {
        let mut left = false;
        let mut down = false;
        let mut right  = false;
        let mut up  = false;
        let mut cw = false;
        let mut ccw = false;

        let mut text_accuum: String = String::new();
        for event in self.pump.poll_iter() {
            match event {
                Event::TextInput{text, ..} => {
                    text_accuum = text;
                },
                Event::MouseButtonDown{x, y, mouse_btn, ..} => {
                  let dims = match self.canvas.output_size() {
                    Ok(f) => f,
                    Err(_e) => (0, 0),
                  };
                  match mouse_btn {
                    MouseButton::Left => {
                      match &self.scene {
                        Scenes::Menu(m) => {
                          for button in &m.buttons {
                              let iwidth = (button.width * dims.0 as f32) as i32;
                              let iheight = (button.height * dims.1 as f32) as i32;
                              let icx = (button.cx * dims.0 as f32) as i32;
                              let icy = (button.cy * dims.1 as f32) as i32;
                              let cornx = icx - iwidth / 2;
                              let corny = icy - iheight / 2;
                              if (x >= cornx && x <= (cornx + iwidth)) &&
                                 (y >= corny && y <= (corny + iheight)) {
                                  (button.callback)();
                              }

                          }
                        },
                        _ => {}
                      }
                    },
                    _ => {},
                  }
                },
                Event::KeyDown{scancode, ..} => {
                    if scancode == Some(Scancode::Grave) {
                        if self.console.is_some() {
                            self.disable_console();
                        } else {
                            self.enable_console();
                        }
                        break;
                    }
                },
                Event::Quit{..} => {
                    return false;
                },
                _ => {},
            }
        }

        match &mut self.console {
            None => {
                //get what keycodes symbolize, we can use client keyboard settings to do that
                //After, serialize and send over
                let kbs = self.pump.keyboard_state();
                left = kbs.is_scancode_pressed(Scancode::A) || kbs.is_scancode_pressed(Scancode::Left);
                down = kbs.is_scancode_pressed(Scancode::S) || kbs.is_scancode_pressed(Scancode::Down);
                right = kbs.is_scancode_pressed(Scancode::D) || kbs.is_scancode_pressed(Scancode::Right);
                up = kbs.is_scancode_pressed(Scancode::W) || kbs.is_scancode_pressed(Scancode::Up);
                ccw = kbs.is_scancode_pressed(Scancode::Q) || kbs.is_scancode_pressed(Scancode::Z);
                cw = kbs.is_scancode_pressed(Scancode::E) || kbs.is_scancode_pressed(Scancode::X);
            },
            Some(c) => {
                c.inp.push_str(&text_accuum);
            },
        }

        match &mut self.scene {
            Scenes::GamePlay(gdata) => {
                //let gpv = gdata.player.vel();
                let rot = gdata.player.rot();

                //if up {
                //    let ddxdt: f64 = rot.to_radians().cos() * ACCEL;
                //    let ddydt: f64 = rot.to_radians().sin() * ACCEL;

                //    gdata.player.change_vel(ddxdt, ddydt);
                //}
                let updown: i8 = if up {-1} else {0} + if down {1} else {0};
                let leftright: i8 = if left {-1} else {0} + if right {1} else {0};
                gdata.player.move_ent(&gdata.grid, leftright, updown);


                //if down {
                //    let dir = gpv.y.atan2(gpv.x);
                //    let mut ddxdt: f64 = -dir.cos() * 1.5 * ACCEL;
                //    let mut ddydt: f64 = -dir.sin() * 1.5 * ACCEL;

                //    //if the signs are not equal and back would cause direction change, set velocity to 0 instead
                //    if gpv.x.signum() != ddxdt.signum() && ddxdt.abs() > gpv.x.abs() {ddxdt = -gpv.x;}
                //    if gpv.y.signum() != ddydt.signum() && ddydt.abs() > gpv.y.abs() {ddydt = -gpv.y;}
                //    gdata.player.change_vel(ddxdt, ddydt);
                //}

                let mut rv: f64 = 0.0;
                if cw { rv += 3.0; }
                if ccw { rv -= 3.0; }
                gdata.player.rotate(rv);

                //loop over all entities, for now we just do player
                //gdata.player.apply_vel(&gdata.grid);
                //propagate changes to the server as well here
            },
            Scenes::Menu(t) => {},
        }
        true
    }
}

