pub use variables_and_mutable::var_and_mut;

pub mod variables_and_mutable {
    pub fn var_and_mut() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");

        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("Three hours is {THREE_HOURS_IN_SECONDS} in seconds");

        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");

        let spaces = "     ";
        let spaces = spaces.len();
        println!("The spaces is {spaces}");
    }
}

pub mod data_types {
    use std::io;

    pub fn integer() {
        let a = 1_i8;
        let b = 2_i16;
        let c = 3_i32;
        let d = 4_i64;
        let e = 5_i128;
        let f = 6_isize;

        println!("{a}");
        println!("{b}");
        println!("{c}");
        println!("{d}");
        println!("{e}");
        println!("{f}");

        println!("----------------");

        let i = 1_u8;
        let j = 2_u16;
        let k = 3_u32;
        let l = 4_u64;
        let m = 5_u128;
        let n = 6_usize;

        println!("{i}");
        println!("{j}");
        println!("{k}");
        println!("{l}");
        println!("{m}");
        println!("{n}");
    }

    pub fn floating_point() {
        let a = 1.0_f32;
        let b = 2.0_f64;

        println!("{a}");
        println!("{b}");
    }

    pub fn operation() {
        let sum = 5 + 10;
        let difference = 95.5 - 4.3;
        let product = 4 * 30;
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;
        let remainder = 43 % 5;

        println!("{sum}");
        println!("{difference}");
        println!("{product}");
        println!("{quotient}");
        println!("{truncated}");
        println!("{remainder}");
    }

    pub fn boolean() {
        let t = true;
        let f = false;

        println!("{t}");
        println!("{f}");
    }

    pub fn character() {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        println!("{c}");
        println!("{z}");
        println!("{heart_eyed_cat}");
    }

    pub fn compound_types() {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("x: {x}");
        println!("y: {y}");
        println!("z: {z}");

        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        println!("{five_hundred}");
        println!("{six_point_four}");
        println!("{one}");
    }

    pub fn array() {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        println!("{first}");

        println!("Please input an array index");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index = index
            .trim()
            .parse::<usize>()
            .expect("Index entered was not a number");
        let element = a[index];

        println!("The value of the element at index {index} is {element}");
    }

    pub mod functions {
        pub fn func_one() {
            println!("Hello world!");
            another_function(5);
        }

        fn another_function(x: i32) {
            println!("The value of x is {x}");
        }

        pub fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {value}{unit_label}");
        }
    }

    pub fn control_flow() {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }
}
