pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn internal_adder(a:i32, b:i32)->i32{
    a + b
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore="expensive"]
    fn expensive_test() {
        sleep(Duration::from_secs(10));
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


#[cfg(test)]
mod tests_internal{
    use super::*;

    #[test]
    fn internal(){
        assert_eq!(4, internal_adder(2,2));
    }
}