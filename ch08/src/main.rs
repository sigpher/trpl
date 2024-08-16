use ch08::Cell;

fn main() {
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
    println!("{row:?}");
}
