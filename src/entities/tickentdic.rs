#[allow(unused_imports)]
use std::time::Duration;

use crate::gameloop::gamestate::GameData;
use crate::gameloop::entities::*;

pub static THICCKENTS: [fn(&mut GameData, usize, Position, Duration) -> TickEnt; 1] = [
    (|gdata: &mut GameData, pid, pos, now| -> TickEnt {
        let p1 = gdata.players[pid].pos();
        let p2 = Position {x: (pos.x - p1.x) / 5.0, y: (pos.y - p1.y) / 5.0};
        TickEnt {
            health: 1.0,
            maxhealth: 1.0,
            velocity: p2,
            maxvelocity: 0.0,
            pos: p1,
            dims: Position {x: 0.25, y: 0.25},
            rot: 0.0,
            state: pid,
            made: now,
            last: now,
            brain: |gdata, eid, now| -> bool {
                let mut me = &mut gdata.tickents[eid];
                if (now - me.made).as_secs_f64() < 5.0 {
                    let t = (now - me.last).as_secs_f64();
                    me.pos.x += me.velocity.x * t;
                    me.pos.y += me.velocity.y * t;
                    me.last = now;
                    for (i,pp) in gdata.players.iter_mut().enumerate() {
                        if i != me.state && pp.health() > 0.0 && me.collide(pp) {
                            pp.damage(6.9);
                            return false
                        }
                    }
                    return true
                }
                false
            },
        }
    })

];
