#![allow(dead_code)]
fn main() {
    let is_even = |x| 0 == x % 2;

    // using for loop 
    // and an if expression
    for num in 0 ..= 10 {
        println!("{:02} is {}", num, if is_even(num) {
            "even number"
        } else {
            "odd number"
        });
    }

    let pairs = Pairs::new("tim", 23);
    println!("{:?}", pairs);
    println!("{} {}", pairs.first(), Pairs {first: 1, second: "circle"}.second());

    // using tuple and others
    let tuple = ("kotlin", "java");
    let lang = Pairs::from_tuple(tuple);
    println!("{:?} {:?} {:?}", tuple, lang, lang.to_tuple());

}

#[derive(Debug, Clone)]
struct Pairs<T, R> {
    first: T,
    second: R,
}

impl <T: Copy, R: Copy> Pairs<T, R> {
    fn new(first: T, second: R) -> Self {
        Pairs {
            first, second
        }
    }

    fn from_tuple(val: (T, R)) -> Self {
        Pairs {
            first: val.0,
            second: val.1
        }
    }

    fn to_tuple(&self) -> (T, R) {
        (self.first(), self.second())
    }

    fn first(&self) -> T {
        self.first
    }

    fn second(&self) -> R {
        self.second
    }
}
