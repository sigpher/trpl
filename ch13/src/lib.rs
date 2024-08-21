#[derive(Debug, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            if let ShirtColor::Red = color {
                num_red += 1;
            } else {
                num_blue += 1;
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub enum Selection<T> {
    Exist(T),
    Null,
}

impl<T> Selection<T> {
    pub fn unwrap(self) -> T {
        match self {
            Selection::Exist(x) => x,
            Selection::Null => panic!("Error"),
        }
    }

    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Selection::Exist(x) => x,
            Selection::Null => f(),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle <'a>{
    pub width: u32,
    pub height: u32,
    pub lable: &'a str,
}