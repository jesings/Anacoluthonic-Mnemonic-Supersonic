use std::time::Duration;

use crate::gameloop::gamestate::GameData;
use crate::gameloop::entities::*;

#[path = "skill/skilldict.rs"]mod skilldict;
    
pub struct Skill {
    cd: Duration,
    up: Duration,
    skill: fn(&mut GameData, Position) -> bool,
    pub skillid: usize, // for like an array of names or textures or something idk?
}

impl Skill {
    pub fn new(skillid:usize) -> Skill {
        Skill {
            cd: Duration::new(skilldict::SKILLS[skillid].1,skilldict::SKILLS[skillid].2),
            up: Duration::new(0,0),
            skill: skilldict::SKILLS[skillid].0,
            skillid: skillid,
        }
    }
    pub fn setcd(&mut self, used: Duration) {
        self.up = used + self.cd;
    }
    pub fn offcd(&self, now: Duration) -> bool {
        now >= self.up
    }
    pub fn percd(&self, now: Duration) -> f32 {
        if self.offcd(now) {0.0} else {(self.up.as_millis() - now.as_millis())as f32 / self.cd.as_millis()as f32}
    }
    pub fn useskill(&mut self, gdata: &mut GameData, pos: Position, now: Duration) -> bool {
        if (self.offcd(now)){
            self.setcd(now);
            return (self.skill)(gdata, pos)
        }
        false
    }
}
