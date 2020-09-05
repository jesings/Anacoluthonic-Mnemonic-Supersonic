use std::sync::{Arc,Mutex};
use std::convert::TryInto;
use std::time::Duration;

use crate::gameloop::gamestate::GameData;
use crate::gameloop::entities::*;
use crate::gameloop::grid::Tile;

pub enum PacketVal {
    Pos(Position),
    Float64(f64),
    Float32(f32),
    Usize(usize),
}

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
            4 => {
                // thicc kent creation
                let te = TickEnt::new(&mut gd,buftousize(&buf[i+1..i+9]),buftousize(&buf[i+9..i+17]),buftopos(&buf[i+17..i+33]),buftoduration(&buf[i+33..i+45]));
                gd.tickents.push(te);
                i+=45;
            },
            _ => {
                return i
            },
        }
    }
    i
}

pub fn encode_player(buf: &mut [u8], i: usize, pid: u8, feild: u8, val: PacketVal) -> usize {
    // 2 pid feild v a l
    match val {
        PacketVal::Pos(n) => {            
            match feild {
                2 | 4 | 5 => {
                    // velocity | position | dimensions
                    buf[i] = 2;
                    buf[i+1] = pid; // todo bad pid values crash server
                    buf[i+2] = feild;
                    postobuf(n, &mut buf[i+3..i+19]);
                    return 19
                },
                _ => {
                    panic!("bad feild for position")
                },

            }
        },
        PacketVal::Float32(n) => {
            match feild {
                0 | 1  => {
                    // health | maxhealth
                    buf[i] = 2;
                    buf[i+1] = pid;
                    buf[i+2] = feild;
                    f32tobuf(n, &mut buf[i+3..i+7]);
                    return 7
                },
                _ => {
                    panic!("bad feild for f32")
                },

            }
        },
        PacketVal::Float64(n) => {
            match feild {
                3 | 6  => {
                    // maxvelocity | rot
                    buf[i] = 2;
                    buf[i+1] = pid;
                    buf[i+2] = feild;
                    f64tobuf(n, &mut buf[i+3..i+11]);
                    return 11
                },
                _ => {
                    panic!("bad feild for f64")
                },

            }
        },
        _ => {
            panic!("bad value")
        }
    }
}

pub fn encode_tickent(buf: &mut [u8], i: usize, eid: usize, pid: usize, pos: Position, now: Duration) {
    buf[i] = 4;
    usizetobuf(eid, &mut buf[i+1..i+9]);
    usizetobuf(pid, &mut buf[i+9..i+17]);
    postobuf(pos, &mut buf[i+17..i+33]);
    durationtobuf(now, &mut buf[i+33..i+45]);
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
fn buftou64(posbuf: &[u8]) -> u64 {
    u64::from_le_bytes(posbuf[0..8].try_into().unwrap())
}
fn u64tobuf(n: u64, posbuf: &mut [u8]) {
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
fn buftou32(posbuf: &[u8]) -> u32 {
    u32::from_le_bytes(posbuf[0..4].try_into().unwrap())
}
fn u32tobuf(n: u32, posbuf: &mut [u8]) {
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
fn postobuf(pos: Position, posbuf: &mut [u8]) {
    for (i,e) in pos.x.to_le_bytes().iter().zip(pos.y.to_le_bytes().iter()).enumerate(){
        posbuf[i] = *e.0;
        posbuf[i+8] = *e.1;
    }
}
fn buftoduration(posbuf: &[u8]) -> Duration {
    Duration::new(buftou64(&posbuf[0..8]),buftou32(&posbuf[0..4]))
}
fn durationtobuf(dur: Duration, posbuf: &mut [u8]) {
    u64tobuf(dur.as_secs(),&mut posbuf[0..8]);
    u32tobuf(dur.subsec_nanos(),&mut posbuf[8..12]);
}
