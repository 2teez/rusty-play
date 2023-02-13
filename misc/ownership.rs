#![allow(dead_code)]

#[derive(Debug)]
struct Person <'a> {
    name: &'a str,
    birth: usize,
}

fn main() {
    let mut ppl = Vec::new();
    ppl.push(Person {name: "timothy", birth: 1987});
    ppl.push(Person {name: "tolulope", birth: 1989});

    println!("{:?}", ppl);
    for person in &ppl {
        println!("{}", print_person(person));
    }
}

fn print_person(person: &Person) -> String {
    format!("In {}, {} was born.", person.birth, person.name)
}
