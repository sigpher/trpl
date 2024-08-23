use ch05::color::Color;

fn main() {
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255 ,0);

    let red_green = Color::mix(&red, &green);
    println!("{red_green:?}");
}
