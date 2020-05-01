use super::entities::*;

pub struct player {
    health: f32,
    maxhealth: f32,
    //equipment we'll handle later I guess
    velocity: Position,
    maxvelocity: Position,
    position: Position,
    hitbox: HitBox,
    //Some kinda sprite here
}

// impl Entity for player {
//     pub fn collide(&self, other: Entity) -> bool{
//     }
//     pub fn changepos(&self, dx: f64, dy: f64) -> bool;
//     pub fn changevel(&self, dx: f64, dy: f64) -> bool;
//     pub fn damage(&self, qty: f32) -> bool;
// }
