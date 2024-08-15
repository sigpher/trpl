#[derive(Debug)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self(a, b, c)
    }
    pub fn distance(point_a: &Point, point_b: &Point) -> f64 {
        f64::sqrt(
            (point_a.0 - point_b.0) * (point_a.0 - point_b.0)
                + (point_a.1 - point_b.1) * (point_a.1 - point_b.1)
                + (point_a.2 - point_b.2) * (point_a.2 - point_b.2),
        )
    }
}
