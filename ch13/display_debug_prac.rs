#![allow(dead_code, unused)]

use std::fmt::{self, Debug, Display, Formatter, Result};

struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
    pub fn say(&self) {
        println!("{:#?} {}", self, self);
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Person(name: [{}], age: [{}])", self.name, self.age)
    }
}

impl Debug for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

fn main() {
    let doe = Person::new("", 0);
    let tim = Person::new("tim", 23);

    doe.say();
    tim.say();
}
