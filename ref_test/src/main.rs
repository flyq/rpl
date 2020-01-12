#![feature(core_intrinsics)]

fn print_type<T>(_args: &T) {
    unsafe {
        println!("{}", std::intrinsics::type_name::<T>());
    }
}

use typename::TypeName;

fn main() {
    println!("Hello, world!");
    let mut x : Option<String> = Some("hello".into());
    let mut y : Option<String> = Some("hello".into());    
    match x {
        Some(ref mut i) => i.push_str(",world"),
        None => println!("none"),
    }


    match &mut y {
        Some(i) => {
            i.push_str(",world");
            println!("{:?}", i);
        },
        None => println!("none"),
    }

    println!("{:?}", x);
    println!("{:?}", y);

    let z:&i32 = &32;
    let &a = z;
    let b = z;
    println!("{}, and typename: {}",a, a.type_name_of());
    println!("{}, and typename: {}",b, b.type_name_of());
    println!("{}, and typename: {}",z, z.type_name_of());
    print_type(&a);
    print_type(&b);
    print_type(&z);
}
