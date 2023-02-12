#![allow(dead_code, unused)]

fn main() {
    println!("{:?}", build_vector());
    println!("{:?}", '8'.to_digit(10));
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
