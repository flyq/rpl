fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let a = String::from("Hello world");

    let word = first_word(&a);

    // s.clear(); error, first delete reference, second delete var;
    
    println!("{}", word);
}
