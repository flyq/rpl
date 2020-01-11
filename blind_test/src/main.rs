#![feature(exclusive_range_pattern)]
fn main() {
    println!("Hello, world!");

    let x = 5;
    match x {
        
        e @ 1 .. 5 | e @ 5 .. 10 => println!("got it {}", e),
        _ => println!("anything"),
    }

}

