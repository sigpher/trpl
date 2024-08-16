use crate::shape::Surface;

#[derive(Debug)]
pub struct Circle {
    radius: f32,
}

impl Surface for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}
