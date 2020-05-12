use super::grid::*;

#[derive(Clone, Copy)]
pub struct Position{pub x: f64, pub y: f64}

pub trait Entity {
    fn dims(&self) -> Position;
    fn mut_health(&mut self) -> &mut f32;
    fn maxhealth(&self) -> f32;
    fn mut_pos(&mut self) -> &mut Position;
    fn pos(&self) -> Position;
    fn mut_vel(&mut self) -> &mut Position;
    fn vel(&self) -> Position;
    fn maxvel(&self) -> f64;
    fn rot(&self) -> f64;
    fn mut_rot(&mut self) -> &mut f64;
    fn getrect_h(&self) -> Vec<Position> {
        let rrot = self.rot().rem_euclid(90.0);
        let mydims = self.dims();
        let fullen = mydims.x.hypot(mydims.y);
        let mut g: Vec<Position> = Vec::with_capacity(4);
        let mypos = self.pos();
        for f in 0..4 {
            let erot = rrot + f as f64 * 90.0;
            g.push(Position {x: erot.cos() * fullen + mypos.x, y: erot.sin() * fullen + mypos.y});
        }
        return g;
    }
    fn collide(&self, other: &dyn Entity) -> bool {
        let myrect = self.getrect_h();
        let urrect = other.getrect_h();

        //https://stackoverflow.com/questions/10962379/how-to-check-intersection-between-2-rotated-rectangles
        for polygon in &[myrect.clone(), urrect.clone()] {
            let mut prev = match polygon.last() { Some(t) => t, _ => { return false; } };
            for curpt in polygon {
                let norm = Position {x: curpt.x - prev.y, y: prev.x - curpt.y};

                let mut mymin = f64::INFINITY; let mut mymax = f64::NEG_INFINITY; 
                for point in &myrect {
                    let proj = norm.x * point.x + norm.y * point.y;
                    mymin = proj.min(mymin);
                    mymax = proj.max(mymax);
                }

                let mut urmin = f64::INFINITY; let mut urmax = f64::NEG_INFINITY;
                for point in &urrect {
                    let proj = norm.x * point.x + norm.y * point.y;
                    urmin = proj.min(urmin);
                    urmax = proj.max(urmax);
                }
                if mymax < urmin || urmax < mymin {return false;}

                prev = curpt;
            }
        }
        true
    }
    fn rotate(&mut self, amt: f64) {
        let mr = self.mut_rot();
        *mr += amt;
    }
    fn change_pos(&mut self, dx: f64, dy: f64, gr: &Grid) -> bool {
        let mp = self.mut_pos();
        mp.x += dx;
        mp.y += dy;
        match gr.grid_coord(mp.x.floor() as usize, mp.y.floor() as usize) {
            Some(t) => {
                if t.passable {
                    return true;
                }
            },
            None => {},
        }
        mp.x -= dx;
        mp.y -= dy;
        false
    }
    fn change_vel(&mut self, dxdt: f64, dydt: f64) -> bool {
        let mxv = self.maxvel();
        let mtv = self.mut_vel();
        mtv.x += dxdt;
        mtv.y += dydt;
        let curvel = mtv.x.hypot(mtv.y);
        if curvel > mxv {
            let dir = mtv.y.atan2(mtv.x);
            mtv.x = mxv * dir.cos();
            mtv.y = mxv * dir.sin();
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
    dims: Position,
    rot: f64,
    //Some kinda sprite here
}

impl Player {
    pub fn new() -> Player {
        return Player {
            health: 100.0,
            maxhealth: 100.0,
            velocity: Position {x: 0.0, y: 0.0},
            maxvelocity: 1.0,
            pos: Position {x: 100.0, y: 100.0},
            dims: Position {x: 0.5, y: 0.5},
            rot: 0.0,
        }
    }
}

impl Entity for Player {
    fn dims(&self) -> Position {
        self.dims
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
    fn pos(&self) -> Position {
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
    fn rot(&self) -> f64 {
        self.rot
    }
    fn mut_rot(&mut self) -> &mut f64 {
        &mut self.rot
    }
}

//pub fn revert_collision<E>(e1: E, e2: E) -> bool
//    where E: Entity {
//    false
//}
