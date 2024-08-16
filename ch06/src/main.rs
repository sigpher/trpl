// use ch10::{Circle, Rectangle, Surface};
// fn remote_local_call(){
//     let circle = Circle::new(10.0);
//     let area = circle.area();
//     println!("{:.2}",area);
//     let rect = Rectangle::new(10.0,20.0);
//     println!("{:.2}",rect.area());
// }

use ch06::{match_control_flow::Coin, route, IpAddKind};

fn main() {
    let four = IpAddKind::V4(192, 168, 100, 10);
    let six = IpAddKind::V6(String::from("::1"));

    route(four);
    route(six);

    let x = 5;
    // let y = Some(10);
    let y: Option<i32> = None;
    if let Some(num) = y {
        let sum = x + num;
        println!("{sum}");
    } else {
        println!("None");
    }

    let coin = Coin::Quarter(ch06::match_control_flow::UsState::Alabama);

    if let Coin::Quarter(state) = coin {
        println!("Quarter from {state:?}");
    }
}
