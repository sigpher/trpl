use std::{
    io,
    process,
};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("Too Big!"),
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                println!("Play again?");
                let mut quit = String::new();
                io::stdin().read_line(&mut quit).unwrap();
                if (quit.trim()) == "quit" {
                    process::exit(0);
                }
                secret_number = rand::thread_rng().gen_range(1..=100);
            }
        }
    }
}
