#![allow(dead_code, unused)]

fn main() {
    // range type specified just for fun
    let alphabet: std::ops::Range<char> = 'a'..'w';

    for letter in alphabet {
        println!("{}", letter);
    }

    // take a slice of the alphabet
    let alphabet = ('a'..'z').collect::<Vec<_>>();
    for letter in &alphabet[10..] {
        print!("{} ", letter);
    }

    println!();

    for letter in &alphabet[..10] {
        print!("{} ", letter);
    }
    println!();

    println!("{:?}", &alphabet[..]);
    println!("{:?}", alphabet);

    let s = "abc012è€";
    for n in 0..s.len() {
        println!("{}: {:?}", n, s.as_bytes()[n]);
    }
}
