#![feature(exclusive_range_pattern)]
fn main() {
    println!("Hello, world!");

    let x = 5;
    match x {
        e @ 1 .. 5 | e @ 5 .. 10 => println!("got it {}", e),
        _ => println!("anything"),
    }

    let y = Some(Some(10));
    println!("{:?}", deep_match(y));

    let z = Some(Some(5));
    println!("{:?}", deep_match(z));
    println!("{:?}", deep_match1(z));

}

fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
    match v {
        Some(r @ Some(1..10)) => r,
        _ => None,
    }
}



fn deep_match1(v: Option<Option<i32>>) -> i32 {
    match v {
        Some(Some( r @ 1..10)) => r,
        _ => 0,
    }
}
