#![allow(dead_code, unused)]

fn main() {
    println!("{:?}", build_vector());
    println!("{:?}", '8'.to_digit(10));

    let words = "A town hall different from Blalablueu...";
    let (first, second) = words.split_at(21);
    println!("{} <=> {}", first, second);

    let t = ("timothy", 23);
    let b = Box::new(t);
    println!("{:?} {:?}", b.0, t.1);
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
