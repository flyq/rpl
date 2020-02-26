use std::borrow::Borrow;
use std::fmt::Display;

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}

fn foo2<T: Borrow<i32> + Display>(a: &T) {
    println!("a is borrowed: {}", a);
}

fn foo3(a: i32) {
    println!("a is borrowed: {}", a);
}

fn foo4(a: &i32) {
    println!("a is borrowed: {}", a);
}

fn main() {
    let mut i = 5;
    
    foo(i);
    foo(&i);

    
    foo2(&i);
    foo2(&mut i);
    // foo2(i);  // error

    // foo3(&i); // error
    foo3(i);

    // foo4(i); // error
    foo4(&i);
}
