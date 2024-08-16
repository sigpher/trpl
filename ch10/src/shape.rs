pub trait Surface {
    fn area(&self) -> f32;
}

pub mod circle;
pub mod rectangle;