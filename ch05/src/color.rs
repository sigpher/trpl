use std::fmt::Display;

#[derive(Debug)]
pub struct Color(u8, u8, u8);
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn mix(color_a: &Color, color_b: &Color) -> Color {
        const FACTOR: f64 = 0.5;
        let r = (color_a.0 as f64 - (color_a.0 as f64 - color_b.0 as f64) * (1.0 - FACTOR)).round();
        let g = (color_a.1 as f64 - (color_a.1 as f64 - color_b.1 as f64) * (1.0 - FACTOR)).round();
        let b = (color_a.2 as f64 - (color_a.2 as f64 - color_b.2 as f64) * (1.0 - FACTOR)).round();
        Color(r as u8, g as u8, b as u8)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:X}{:X}{:X}", self.0, self.1, self.2)
    }
}
