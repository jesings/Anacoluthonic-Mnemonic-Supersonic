use super::grid::*;

#[derive(Clone, Copy)]
pub struct Position{pub x: f64, pub y: f64}

#[derive(Clone, Copy)]
pub struct HitRect{pub x1: f64, pub y1: f64, pub x2: f64, pub y2: f64}

pub trait Entity {
    fn hitrect(&self) -> HitRect;
    fn mut_health(&mut self) -> &mut f32;
    fn maxhealth(&self) -> f32;
    fn mut_pos(&mut self) -> &mut Position;
    fn pos(&mut self) -> Position;
    fn mut_vel(&mut self) -> &mut Position;
    fn vel(&self) -> Position;
    fn maxvel(&self) -> f64;
    fn collide(&self, other: &dyn Entity) -> bool {
        let myrect = self.hitrect();
        let urrect = other.hitrect();
        return myrect.x1 < urrect.x2 && myrect.x2 > urrect.x1 &&
               myrect.y1 < urrect.y2 && myrect.y2 > urrect.y1;
    }
    fn change_pos(&mut self, dx: f64, dy: f64, gr: &Grid) -> bool {
        let mp = self.mut_pos();
        mp.x += dx;
        mp.y += dy;
        //if grid square is not okay, subtract dx, dy back, return false
        true
    }
    fn change_vel(&mut self, dxdt: f64, dydt: f64) -> bool {
        let mxv = self.maxvel();
        let mtv = self.mut_vel();
        mtv.x += dxdt;
        mtv.y += dydt;
        let curvel = mtv.x.hypot(mtv.y);
        if curvel > mxv {
            mtv.x *= curvel / mxv;
            mtv.y *= curvel / mxv;
        }
        true
    }
    fn apply_vel(&mut self, gr: &Grid) -> bool {
        let v = self.vel();
        let v0 = v.x;
        let v1 = v.y;
        let mut mp = self.mut_pos();
        mp.x += v0;
        mp.y += v1;
        match gr.grid_coord(mp.x.floor() as usize, mp.y.floor() as usize) {
            Some(t) => {
                if t.passable {
                    return true;
                }
            },
            None => {},
        }
        mp.x -= v0;
        mp.y -= v1;

        {
            let vm = self.mut_vel();
            if vm.x.abs() > 1.0 || vm.y.abs() > 1.0 {
                if vm.x.abs() >= vm.y.abs() {
                    vm.x = 0.0;
                } else {
                    vm.y = 0.0;
                }
                return false;
            } 
        }

        mp = self.mut_pos();
        let mut xpass = false;
        let mut ypass = false;

        match gr.grid_coord((mp.x + v0).floor() as usize, mp.y.floor() as usize) {
            Some(t) => xpass = t.passable,
            None => {},
        }
        match gr.grid_coord(mp.x.floor() as usize, (mp.y + v1).floor() as usize) {
            Some(t) => ypass = t.passable,
            None => {},
        }
        if xpass && !ypass { mp.x += v0; }
        else if ypass && !xpass { mp.y += v1; }
        let vm = self.mut_vel();
        if !xpass { vm.x = 0.0; }
        if !ypass { vm.y = 0.0; }
        false
    }
    fn damage(&mut self, qty: f32) -> bool {
        let max = self.maxhealth();
        let healthnow = self.mut_health();
        *healthnow -= qty;
        if *healthnow > max {
            *healthnow = max
        }
        *healthnow > 0.0
    }
}

pub struct Player {
    health: f32,
    maxhealth: f32,
    //equipment we'll handle later I guess
    velocity: Position,
    maxvelocity: f64,
    pos: Position,
    pub hitbox: HitRect,
    //Some kinda sprite here
}

impl Player {
    pub fn new() -> Player {
        return Player {
            health: 100.0,
            maxhealth: 100.0,
            velocity: Position {x: 0.0, y: 0.0},
            maxvelocity: 10.0,
            pos: Position {x: 100.0, y: 100.0},
            hitbox: HitRect {x1: 0.5, y1: 0.5, x2: 0.5, y2: 0.5},
        }
    }
}

impl Entity for Player {
    fn hitrect(&self) -> HitRect {
        HitRect {
            x1: self.pos.x - self.hitbox.x1,
            y1: self.pos.y - self.hitbox.y1,
            x2: self.pos.x + self.hitbox.x2,
            y2: self.pos.y + self.hitbox.y2
        }
    }
    fn mut_health(&mut self) -> &mut f32 {
        &mut self.health
    }
    fn maxhealth(&self) -> f32 {
        self.maxhealth
    }
    fn mut_pos(&mut self) -> &mut Position {
        &mut self.pos
    }
    fn pos(&mut self) -> Position {
        self.pos
    }
    fn mut_vel(&mut self) -> &mut Position {
        &mut self.velocity
    }
    fn vel(&self) -> Position {
        self.velocity
    }
    fn maxvel(&self) -> f64 {
        self.maxvelocity
    }
}

