#[allow(unused_imports)]
use std::time::Duration;

use crate::gameloop::entities::*;
use crate::gameloop::gamestate::GameData;

pub static SKILLS: [(fn(&mut GameData, Position) -> bool, u64, u32, bool); 1] = [
    (|gdata: &mut GameData, pos| -> bool {
        let ppos = gdata.players[gdata.pid as usize].mut_pos();
        ppos.x = pos.x;
        ppos.y = pos.y;
        true
    }, 5, 0, true)






];
