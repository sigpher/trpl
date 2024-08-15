use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        let mut guess = String::new();
        println!("Please input your guess");
        io::stdin().read_line(&mut guess).unwrap();

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}
