fn main() {
    let v = vec![1,2,3,4,5];

//    let third: &i32 = &v[6];

//    println!("the third element is {}", third);

    match v.get(6) {
    	Some(third) => println!("{}", third),
	None => println!("no third"),
    }
}
