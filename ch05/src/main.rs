use ch05::{color::Color, geometry::Point};

fn main() {
    let white = Color::new(0xFF, 0xFF, 0xFF);
    let blue = Color::new(0x15, 0x91, 0xDB);

    let mix_color = Color::mix(&white, &blue);
    println!("{mix_color:X?}");
    println!("{}", mix_color);

    let p_a = Point::new(0.0, 0.0, 0.0);
    let p_b = Point::new(2.0, 2.0, 2.0);
    println!("{:.6}", Point::distance(&p_a, &p_b));
    println!("{:.6}", Point::distance(&p_b, &p_a));

    let red = Color::new(250, 0, 0);
    let blue = Color::new(0, 250 ,0);

    let red_blue = Color::mix(&red, &blue);
    println!("{red_blue:?}");
}
