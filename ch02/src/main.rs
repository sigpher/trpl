use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        let mut guess = String::new();
        println!("Please input your guess");
        stdin().read_line(&mut guess).unwrap();
        println!("{guess}");
        let guess = guess.trim().parse::<u32>().unwrap();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
            Ordering::Greater => println!("To Big"),
        }
    }
}
