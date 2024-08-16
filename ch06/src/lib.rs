#[derive(Debug)]
pub enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn route(ip_kind: IpAddKind) {
    println!("{:?}", ip_kind);
}

pub mod match_control_flow {
    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    pub fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter form {state:?}");
                25
            }
        }
    }
    #[derive(Debug)]
    pub enum UsState {
        Alabama,
        Alaska,
    }
}
