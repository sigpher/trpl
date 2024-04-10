fn main() {
    let mut user1 = User {
        active: true,
        email: "someone@example.com".into(),
        username: "Someusername123".into(),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    AlwaysEqual::print_mesage();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;

trait Message {
    fn print_mesage() {
        println!("I am who i am")
    }
}

impl Message for AlwaysEqual {
    fn print_mesage() {
        println!("I am always equal to others")
    }
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.height && self.height > other.height
    }
}
