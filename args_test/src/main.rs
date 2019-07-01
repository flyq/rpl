use std::env;

fn main() {
    let args = env::args();
    //使用 {:?} 来输出
    println!("{:?}", args);
}