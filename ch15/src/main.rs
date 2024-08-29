use std::ops::Deref;

use ch15::{
    CustomSmartPointer,
    List::{Cons, Nil},
    MyBox,
};

fn main() {
    // let b = Box::new(5);
    // println!("{b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{list:?}");

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));

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
}
