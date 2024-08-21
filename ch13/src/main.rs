use std::thread;

use ch13::{
    Inventory, Rectangle,
    Selection::{self, Exist, Null},
    ShirtColor::{self, Blue, Red},
};

fn main() {
    let store = Inventory {
        shirts: vec![Blue, Red, Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // pub fn add_one_v1(x: u32) -> u32 {x + 1}
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // let add_one_v3 = |x| x + 1;

    // let add_one_v4 = |x| x + 1;

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();

    println!("After calling closure: {list:?}");

    println!("------------------------------------");

    let mut list_mut = vec![1, 2, 3];
    println!("Before defining closure: {list_mut:?}");
    let mut borrows_mutably = || list_mut.push(10);
    borrows_mutably();
    println!("After defining closure: {list_mut:?}");

    let list_move = vec![1, 2, 3];
    println!("Before defining closure: {list_move:?}");

    thread::spawn(move || println!("From thread: {list_move:?}"))
        .join()
        .unwrap();

    let s1 = Exist(Vec::new().push(1));
    let s2: Selection<Vec<i32>> = Null;

    let s1_value = s1.unwrap();
    let s2_value: Vec<_> = s2.unwrap_or_else(|| Vec::new());

    println!("{s1_value:?}");
    println!("{:?}", s2_value);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
            lable: "wide size",
        },
        Rectangle {
            width: 3,
            height: 5,
            lable: "medium size",
        },
        Rectangle {
            width: 7,
            height: 12,
            lable: "large size",
        },
    ];

    list.sort_by_key(|r| r.width);

    println!("{list:?}");

    list.sort_by_key(|r| r.height);

    println!("{list:?}");

    list.sort_by_key(|r: &Rectangle| r.lable);

    println!("{list:?}");
}
