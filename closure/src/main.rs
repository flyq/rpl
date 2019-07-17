fn main() {
    let num = String::from("test");

    {
        let add_num = move |x: String| x;

        println!("{}", add_num(num));
    }

    // println!("{}", num);
    println!("{}", "can't print num");
}
