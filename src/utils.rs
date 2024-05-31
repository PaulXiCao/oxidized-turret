#[derive(Clone, Copy)]
pub struct FloatPosition {
    pub x: f32,
    pub y: f32,
}

pub fn distance(a: FloatPosition, b: FloatPosition) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx.powi(2) + dy.powi(2)).sqrt()
}
