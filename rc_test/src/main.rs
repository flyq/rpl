use std::rc::Rc;

fn main() {
    let r1 = Rc::new(1);
    
    println!("reference count r1  {}", Rc::strong_count(&r1));
    let r2 = r1.clone();
    println!("reference count  r1 {}", Rc::strong_count(&r1));
    println!("reference count r2 {}", Rc::strong_count(&r2));
}
