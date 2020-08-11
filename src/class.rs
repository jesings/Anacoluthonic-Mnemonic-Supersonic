use std::time::Duration;

use crate::gameloop::{skill::*,gamestate::*,entities::Position};

pub struct Class {
    pub id: usize,
    pub name: String,
    pub skills: Vec<Skill>,
// texture, 
    
}

impl Class {
    pub fn new(id: usize) -> Class {
        let mut sksk: Vec<Skill> = Vec::new();
        sksk.push(Skill::new(0));
        Class {
            id: id,
            name: "asd".to_string(),
            skills: sksk,
        }
    }
    pub fn use_handle(x: i32, y: i32, now: Duration) -> impl Fn(&mut GameState) -> bool {
        move |gs: &mut GameState| -> bool {
            let n = match &gs.scene {
                Scenes::GamePlay(c) => {match &c {
                    GameplayScene::Skill(n) => {*n},
                    _ => {return false},
                }},
                _ => {return false},
            };
            gs.scene = Scenes::GamePlay(GameplayScene::None);
            if n < gs.class.as_ref().unwrap().skills.len() {
                let mut gd = gs.gamedata.lock().unwrap();
                println!("ok so rn this is just using the coords of where you click with the mouse for this jonathicc u need to actually convert into rrelative cooords pls :)))){} {}",x,y);
                return gs.class.as_mut().unwrap().skills[n].useskill(&mut gd, Position {x: x as f64, y: y as f64}, now)
            }
            false
        }
    }
}
