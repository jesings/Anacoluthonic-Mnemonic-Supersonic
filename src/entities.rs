#[path = "grid.rs"] mod grid;

#[path = "entities/player.rs"] mod player;

pub struct Position(f64, f64);
pub struct HitRect(f64, f64, f64, f64);

pub trait Entity {
    fn hitrect(&self) -> HitRect;
    fn mut_health(&mut self) -> &mut f32;
    fn maxhealth(&self) -> f32;
    fn mut_pos(&mut self) -> &mut Position;
    fn mut_vel(&mut self) -> &mut Position;
    fn vel(&self) -> &Position;
    fn maxvel(&self) -> f64;
    fn collide(&self, other: &dyn Entity) -> bool {
        let myrect = self.hitrect();
        let urrect = other.hitrect();
        return myrect.0 < urrect.2 && myrect.2 > urrect.0 &&
               myrect.1 < urrect.3 && myrect.3 > urrect.1;
    }
    fn change_pos(&mut self, dx: f64, dy: f64, gr: &grid::Grid) -> bool {
        let mp = self.mut_pos();
        mp.0 += dx;
        mp.1 += dy;
        //if grid square is not okay, subtract dx, dy back, return false
        true
    }
    fn change_vel(&mut self, dxdt: f64, dydt: f64) -> bool {
        let mxv = self.maxvel();
        let mtv = self.mut_vel();
        mtv.0 -= dxdt;
        mtv.1 -= dydt;
        let curvel = (mtv.0 * mtv.0 + mtv.1 * mtv.1).sqrt();
        if curvel > mxv {
            mtv.0 *= curvel / mxv;
            mtv.1 *= curvel / mxv;
        }
        true
    }
    fn apply_vel(&mut self, gr: &grid::Grid) -> bool {
        let v = self.vel();
        let v0 = v.0;
        let v1 = v.1;
        let mp = self.mut_pos();
        mp.0 += v0;
        mp.1 += v1;
        //if grid square is not okay, subtract dx, dy back, return false
        true
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
