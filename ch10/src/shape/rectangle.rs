use crate::shape::Surface;

#[derive(Debug)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Surface for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
