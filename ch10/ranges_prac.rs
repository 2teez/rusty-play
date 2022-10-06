#![allow(dead_code, unused)]

fn main() {
    // range type specified just for fun
    let alphabet: std::ops::Range<char> = 'a'..'w';

    for letter in alphabet {
        println!("{}", letter);
    }
}
