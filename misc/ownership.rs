#![allow(dead_code)]

#[derive(Debug)]
struct Person <'a> {
    name: &'a str,
    birth: usize,
}

#[derive(Debug, Clone)]
struct PPerson {
    name: String,
    birth: i32,
}

fn main() {
    let mut ppl = Vec::new();
    ppl.push(Person {name: "timothy", birth: 1987});
    ppl.push(Person {name: "tolulope", birth: 1989});

    println!("{:?}", ppl);
    for person in &ppl {
        println!("{}", print_person(person));
    }

    let kunle = PPerson {name: "kunle".to_string(), birth: 1980};
    print!("{}", print_pperson(kunle.clone()));
    println!("{:?}", kunle);
}

fn print_person(person: &Person) -> String {
    format!("In {}, {} was born.", person.birth, person.name)
}

fn print_pperson(person: PPerson) -> String {
    format!("In {}, {} was born.", person.birth, person.name)
}
