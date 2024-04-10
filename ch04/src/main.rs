fn main() {
    let s = "how world";
    println!("{:?}", first_word(s));
}

fn change(some_thing: &mut String) {
    some_thing.push_str(",afdafaaaaaaaaaa");
}

fn first_word(s: &str) -> &str {
    for a in s.as_bytes().iter().enumerate().map(|(x, y)| (x, y)) {
        if *a.1 == b' ' {
            return &s[..a.0];
        }
    }
    &s[..]
}
