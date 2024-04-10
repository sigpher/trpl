use std::io;

pub fn types() {
    // let x = 5;
    let mut x = 5;
    println!("THe value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("{THREE_HOURS_IN_SECONDS}");
    {
        let x = x * 2;
        println!("Inner x: {x}");
    }
    println!("Outer x: {x}");

    let space = "     ";
    let space = space.len();
    println!("{space}");

    let guess = "42".parse::<u8>().expect("Not a number");
    println!("{:?}", guess);

    let tup = (500f32, 6.4f32, 1u32);
    println!("{tup:?}");

    let array = [1, 2, 3, 4, 5];
    array.map(|e| e * 2).iter().for_each(|e| {
        println!("{e}");
    });

    let array2 = [3u8; 5];
    array2.map(|e| e * 2).iter().for_each(|e| {
        println!("{e}");
    });

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();

    let index = index.trim().parse::<usize>().unwrap();
    let element = array.get(index).unwrap_or_else(|| {
        println!("wrong index, we'll give you the 0 instead");
        &0
    });
    println!("{}", element);
}
