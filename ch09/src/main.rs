use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let usernames = read_usernames_from_file()?;
    println!("{usernames:?}");
    Ok(())
}

fn read_usernames_from_file() -> Result<Vec<String>,  Box<dyn Error>> {
    let mut vec_usernames = Vec::new();
    let usernames = fs::read_to_string("username.txt").unwrap();
    let names: Vec<&str> = usernames.split_whitespace().collect();
    for name in names {
        vec_usernames.push(String::from(name))
    }
    Ok(vec_usernames)
}
