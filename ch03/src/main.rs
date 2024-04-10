fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF");
}

fn another_function(x: i32) {
    println!("another function with value: {}", x);
    another_function(10);
    let y = {
        let x = 3;
        x + 1
    };

    let condition = true;
    let z = if condition { 10 } else { 100 };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100000 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
    for i in (1..4).rev(){
        println!("{i}");
    }
}
