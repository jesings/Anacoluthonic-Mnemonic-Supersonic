use sdl2::render::{WindowCanvas};
//use sdl2::pixels::Color;
use sdl2::keyboard::Scancode;
//use sdl2::keyboard::KeyboardState;
use sdl2::mouse::*;
use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::ops::DerefMut;
use std::time::Duration;

use super::grid::{Grid, DTILEDIM};
use super::entities::{Player, Entity, TickEnt};
use super::console::*;
use super::menu::{Button, Slider};
use super::hud::{HudItem, HudText};
use super::packet::*;
use super::class::Class;

static ACCEL: f64 = 1.0 / 64.0;
pub struct GameData {
    pub grid: Option<Grid>,
    pub tickents: Vec<TickEnt>,
    pub players: Vec<Player>,
    pub pid: usize, // pos of clients player in player vecotr
    pub buf: [u8; 4096], // should be periodically updated with changes to gamestate encoded as packets
    pub bufpos: usize, // position of next write into buf
    pub ingame: bool, // marks eol 4 threds
}
pub struct MenuItems {
    pub name: String,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    //??? the above need function callbacks, not sure about click and drag for sliders
}

pub enum GameplayScene { // context for opening menus, targeting skills or items, etc.
    None,
    Inventory,
    Skill(usize),
    Item(usize),
    Entity(usize),
}

pub enum Scenes {
    Menu(MenuItems),
    GamePlay(GameplayScene),
    //No Clue what to put here
}

pub struct GameState<'ttf, 'a> {
    pub canvas: WindowCanvas,
    pub pump: sdl2::EventPump,
    pub console: Option<Console>,
    pub fonts: HashMap<String, Font<'ttf, 'a>>,
    pub vidsub: VideoSubsystem,
    pub scene: Scenes,
    pub huditems: Vec<HudItem>,
    pub hudtexts: Vec<HudText>,
    pub class: Option<Class>,
    pub gamedata: Arc<Mutex<GameData>>,
    pub address: String,
}

pub enum Callback {
    B(fn(&mut GameState) -> bool),
    S(fn(&mut GameState) -> bool),
}

impl GameState<'_, '_> {
    pub fn update(&mut self, now: Duration) -> bool {
        let mut left = false;
        let mut down = false;
        let mut right  = false;
        let mut up  = false;
        let mut cw = false;
        let mut ccw = false;

        let mut text_accuum: String = String::new();
        let mut bcallbacks = vec!();
        let mut scallbacks = vec!();
        let mut ucallbacks = vec!();
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
                                                bcallbacks.push(button.callback);
                                            }

                                    }
                                },
                                Scenes::GamePlay(c) => {
                                    match &c {
                                        GameplayScene::Skill(_) => {
                                            let relx = (x - (dims.0 / 2) as i32) as f64 / DTILEDIM;
                                            let rely = (y - (dims.1 / 2) as i32) as f64 / DTILEDIM;
                                            ucallbacks.push(Class::use_handle(relx, rely, now));
                                        },
                                        _ => {},
                                    }
                                },
                                //_ => {}
                            }
                        },
                        MouseButton::Right => {
                            match &self.scene {
                                Scenes::GamePlay(_) => {
                                    scallbacks.push(GameState::change_gameplayscene(GameplayScene::None));
                                },
                                _ => {},
                            }
                        }
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
                    match &self.scene {
                        Scenes::GamePlay(_) => {
                            match scancode { Some(code) => { match code {
                                Scancode::Num1 => {
                                    scallbacks.push(GameState::change_gameplayscene(GameplayScene::Skill(0)));
                                },
                                _ => {},
                            }}, _ => {},}
                        },
                        _ => {},
                    }
                },
                Event::Quit{..} => {
                    self.gamedata.lock().unwrap().ingame = false;
                    return false;
                },
                _ => {},
            }
        }

        for callback in bcallbacks { // almost, but not quite, all the same type :(( maybe use an enum? but weird types not sure if worth
            (callback)(self);
        }
        for callback in scallbacks {
            (callback)(self);
        }
        for callback in ucallbacks {
            (callback)(self);
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
            Scenes::GamePlay(_) => {
                let mut g = self.gamedata.lock().unwrap();
                let mut gdata = g.deref_mut();
                //let gpv = gdata.player.vel();
                //let rot = gdata.players[gdata.pid].rot();

                //if up {
                //    let ddxdt: f64 = rot.to_radians().cos() * ACCEL;
                //    let ddydt: f64 = rot.to_radians().sin() * ACCEL;

                //    gdata.player.change_vel(ddxdt, ddydt);
                //}
                let updown: i8 = if up {-1} else {0} + if down {1} else {0};
                let leftright: i8 = if left {-1} else {0} + if right {1} else {0};
                gdata.players[gdata.pid].move_ent(&gdata.grid.as_mut().unwrap(), leftright, updown);
                gdata.bufpos += encode_player(&mut gdata.buf, gdata.bufpos, gdata.pid as u8, 4, PacketVal::Pos(gdata.players[gdata.pid].pos()));
                
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
                if cw { rv += 3.0;}
                if ccw { rv -= 3.0;}
                gdata.players[gdata.pid].rotate(rv);

                //loop over all entities, for now we just do player
                //gdata.player.apply_vel(&gdata.grid);
                //propagate changes to the server as well here

            },
            Scenes::Menu(_t) => {},
        }
        true
    }
    fn change_gameplayscene(e: GameplayScene) -> impl Fn(&mut GameState) -> bool {
        move |gs: &mut GameState| -> bool {
            match e {
                GameplayScene::Skill(n) => {
                    if gs.class.as_ref().unwrap().skills.len() > n {
                        gs.scene = Scenes::GamePlay(GameplayScene::Skill(n));
                    }
                    return true
                },
                GameplayScene::None => {
                    gs.scene = Scenes::GamePlay(GameplayScene::None)
                }
                _ => {},
            }
            true
        }
        
    }
}

