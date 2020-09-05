#[allow(unused_imports)]
use std::time::Duration;

use crate::gameloop::entities::*;
use crate::gameloop::gamestate::GameData;
use crate::gameloop::packet::*;

pub static SKILLS: [(fn(&mut GameData, Position, Duration) -> bool, u64, u32, bool); 2] = [
    (|gdata: &mut GameData, pos, now| -> bool { // teleport
        let _ppos = gdata.players[gdata.pid as usize].change_pos(pos.x, pos.y, &gdata.grid.as_ref().unwrap());
        true
    }, 5, 0, true),
    (|gdata: &mut GameData, pos, now| -> bool { // spawn straight line moving entity
        let ppos = gdata.players[gdata.pid as usize].pos();
        let e = TickEnt::new(gdata, 0, gdata.pid as usize, Position {x: pos.x + ppos.x, y: pos.y + ppos.y}, now);
        encode_tickent(&mut gdata.buf, gdata.bufpos, 0, gdata.pid as usize, Position {x: pos.x + ppos.x, y: pos.y + ppos.y}, now);
        gdata.bufpos+=45;
        gdata.tickents.push(e);
        true
    }, 1, 0, true)
        





];
