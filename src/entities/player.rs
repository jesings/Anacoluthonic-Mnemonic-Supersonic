use super::*;

pub struct Player<'a> {
    health: &'a mut f32,
    maxhealth: &'a mut f32,
    //equipment we'll handle later I guess
    velocity: &'a mut Position,
    maxvelocity: &'a mut f64,
    pos: &'a mut Position,
    hitbox: &'a mut HitRect,
    //Some kinda sprite here
}

impl Entity for Player<'_> {
    fn hitrect(&self) -> HitRect {
        HitRect(
            self.pos.0 - self.hitbox.0,
            self.pos.1 - self.hitbox.1,
            self.pos.0 + self.hitbox.2,
            self.pos.1 + self.hitbox.3
        )
    }
    fn mut_health(&mut self) -> &mut f32 {
        &mut self.health
    }
    fn maxhealth(&self) -> f32 {
        *self.maxhealth
    }
    fn mut_pos(&mut self) -> &mut Position {
        &mut self.pos
    }
    fn mut_vel(&mut self) -> &mut Position {
        &mut self.velocity
    }
    fn vel(&self) -> &Position {
        self.velocity
    }
    fn maxvel(&self) -> f64 {
        *self.maxvelocity
    }
}
