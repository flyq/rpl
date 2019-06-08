use typename::TypeName;
use std::io;

fn main() {
    let a = "hello";
    let b = String::from("hello");
    let mut c = String::new();
    
    println!("Please enter hello");
    io::stdin().read_line(&mut c)
        .expect("Failed to read line");

    println!("the type of {} is {}, and len is {}", a, a.type_name_of(), a.len());
    println!("the type of {} is {}, and len is {}", b, b.type_name_of(), b.len());
    println!("the type of {} is {}, and len is {}", c, c.type_name_of(), c.len());
}
