#![allow(dead_code, unused)]

#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    pub fn naming(&self) -> String {
        format!("{}, {}", self.first_name, self.last_name)
    }

    pub fn change_last_name(&mut self, nname: &'a str) -> Self {
        self.last_name = nname;
        Person {
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

type Str = String;
type Num = u8;

mod human {

    #[derive(Debug)]
    pub struct Person {
        name: super::Str,
        age: crate::Num,
    }

    impl Person {
        pub fn new(name: crate::Str, age: crate::Num) -> Person {
            Self { name, age }
        }

        pub fn new_age(&self, age: super::Num) -> Self {
            Person {
                name: self.name.to_owned(),
                age,
            }
        }

        pub fn pprnt(&self) {
            println!(
                "{}",
                format!("Human::Person(name: {}, age: {})", self.name, self.age)
            );
        }
    }
}

fn main() {
    let mut tim = Person {
        first_name: "tim",
        last_name: "adigun",
    };

    println!("{:?} {:#?}", tim.naming(), Person::naming(&tim));
    Person::change_last_name(&mut tim, "emrys");
    println!("{:?}", tim);

    // using a mod and function
    let tolu = human::Person::new("tolu".to_string(), 18);
    println!("{:#?}", tolu);
    tolu.new_age(21).pprnt();
}
