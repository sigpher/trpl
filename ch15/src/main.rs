use std::{ops::Deref, rc::Rc};

use ch15::{
    hello, CustomSmartPointer,
    List::{Cons, Nil},
    MyBox,
};

fn main() {
    // let b = Box::new(5);
    // println!("{b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{list:?}");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // println!("count after c goes out of scope = {}", Rc::weak_count(&a));

    let x = 5;
    let y = &x;

    assert_eq!(x, *y);

    let z = Box::new(5);
    assert_eq!(x, *z);

    let my_box = MyBox::new(10);

    assert_eq!(10, *my_box);
    assert_eq!(10, *(my_box.deref()));

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop();
    drop(c);
    println!("CustomSmartPointers created.");

    let username = Box::new(String::from("choi"));
    hello(&username);

    let i = 4;
    let j = &mut i;
}
