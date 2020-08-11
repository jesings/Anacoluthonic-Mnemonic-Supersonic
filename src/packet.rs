use std::sync::{Arc,Mutex};
use std::convert::TryInto;

use crate::gameloop::gamestate::GameData;
use crate::gameloop::entities::*;
use crate::gameloop::grid::Tile;

pub fn packet_decode(buf: &[u8], gdata: Arc<Mutex<GameData>>) -> usize {
    let mut i: usize = 1; // zero index is pid for server propagation
    while i<buf.len() {
        let mut gd = gdata.lock().unwrap();    
        match buf[i] {
            // initial value zero should be used to mark end of packet
            1 => {
                // grid
                let mut t: &mut Tile = match gd.grid.as_mut().unwrap().mut_grid_coord(buftousize(&buf[i+1..i+9]),buftousize(&buf[i+9..i+17])) {
                    Some(g) => g,
                    None => return i,
                };
                t.texture = buf[i+17];
                t.passable = buf[i+18]!=0;
                i+=19;
            },
            2 => {
                // players
                let e = buf[i+1] as usize;
                match buf[i+2] {
                    0 => {
                        // health
                        *gd.players[e].mut_health() = buftof32(&buf[i+3..i+7]);
                        i+=7;
                    },
                    1 => {
                        // maxhealth
                        *gd.players[e].mut_maxhealth() = buftof32(&buf[i+3..i+7]);
                        i+=7;
                    },
                    2 => {
                        // velocity
                        *gd.players[e].mut_vel() = buftopos(&buf[i+3..i+19]);
                        i+=19;
                    },
                    3 => {
                        // maxvelocity
                        *gd.players[e].mut_maxvel() = buftof64(&buf[i+3..i+11]);
                        i+=11;
                    },
                    4 => {
                        // position
                        *gd.players[e].mut_pos() = buftopos(&buf[i+3..i+19]);
                        i+=19;
                    },
                    5 => {
                        // dims
                        *gd.players[e].mut_dims() = buftopos(&buf[i+3..i+19]);
                        i+=19;
                    },
                    6 => {
                        // rot
                        *gd.players[e].mut_rot() = buftof64(&buf[i+3..i+11]);
                        i+=11;
                    },
                    _ => {
                        return i
                    },
                }
            },
            3 => {
                // ingame
                gd.ingame = buf[i+1]!=0;
                i+=2;
            },
            _ => {
                return i
            },
        }
    }
    i
}

pub fn encode_player_pos(buf: &mut [u8], i: usize, pid: u8, feild: u8, val: &Position) -> usize {
    // 2 pid feild v a l
    match feild {
        2 | 4 | 5 => {
            // velocity | position | dimensions
            buf[i] = 2;
            buf[i+1] = pid; // todo bad pid values crash server
            buf[i+2] = feild;
            postobuf(val, &mut buf[i+3..i+19]);
            return 19;
        },
        _ => {
            panic!("bad feild for position")
        },
    }
}

// le bytes conversion helpers
fn buftousize(posbuf: &[u8]) -> usize {
    usize::from_le_bytes(posbuf[0..8].try_into().unwrap())
}
fn usizetobuf(n: usize, posbuf: &mut [u8]) {
    for (i,e) in n.to_le_bytes().iter().enumerate() {
        posbuf[i] = *e;
    }
}
fn buftof64(posbuf: &[u8]) -> f64 {
    f64::from_le_bytes(posbuf[0..8].try_into().unwrap())
}
fn f64tobuf(n: f64, posbuf: &mut [u8]) {
    for (i,e) in n.to_le_bytes().iter().enumerate() {
        posbuf[i] = *e;
    }
}
fn buftof32(posbuf: &[u8]) -> f32 {
    f32::from_le_bytes(posbuf[0..4].try_into().unwrap())
}
fn f32tobuf(n: f32, posbuf: &mut [u8]) {
    for (i,e) in n.to_le_bytes().iter().enumerate() {
        posbuf[i] = *e;
    }
}
fn buftopos(posbuf: &[u8]) -> Position {
    Position {
        x: buftof64(&posbuf[0..8]),
        y: buftof64(&posbuf[8..16]),
    }
}
fn postobuf(pos: &Position, posbuf: &mut [u8]) {
    for (i,e) in pos.x.to_le_bytes().iter().zip(pos.y.to_le_bytes().iter()).enumerate(){
        posbuf[i] = *e.0;
        posbuf[i+8] = *e.1;
    }
}
