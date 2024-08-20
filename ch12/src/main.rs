use std::{env, process};

use ch12::{run, Config};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
       eprintln!("Application error: {e}");
        process::exit(1)
    }
}

