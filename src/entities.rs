struct Position(f64, f64);
struct HitBox(f64, f64, f64, f64);

pub trait Entity {
    pub fn collide(&self, other: Entity) -> bool;
    pub fn changepos(&self, dx: f64, dy: f64) -> bool;
    pub fn changevel(&self, dx: f64, dy: f64) -> bool;
    pub fn damage(&self, qty: f32) -> bool;
}
