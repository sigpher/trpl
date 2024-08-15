use ch05::{color::Color, geometry::Point};

fn main() {
    let white = Color::new(0xFFu8, 0xFFu8, 0xFFu8);
    let blue = Color::new(0x15, 0x91, 0xDB);

    let mix_color = Color::mix(&white, &blue);
    println!("{mix_color:X?}");
    println!("{}", mix_color);

    let p_a = Point::new(0.0, 0.0, 0.0);
    let p_b = Point::new(2.0, 2.0, 2.0);
    println!("{:.6}", Point::distance(&p_a, &p_b));
    println!("{:.6}", Point::distance(&p_b, &p_a));
}
