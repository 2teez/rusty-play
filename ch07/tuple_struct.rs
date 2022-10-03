// a struct like tuple
// can be used like namedtuple in python
#[derive(Debug)]
struct Persons<'a>(&'a str, u8, char);

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u8,
    gender: char,
}

fn main() {
    let tomi = ("tomi", 15, "female");
    let kunle: (&str, u8, &str) = ("kunle", 18, "male");

    println!("{:#?}, {:#?}", tomi, kunle);
    println!("Age: {}", kunle.1);

    let tomi = Persons("tomi", 34, 'F');
    println!("{:#?}", tomi);
    println!("Name: {}, Gender: {}", tomi.0, tomi.2);

    let mut tim = Person {
        name: "timothy",
        age: 23,
        gender: 'M',
    };
    println!("{:#?}", tim);
    tim.age = 40;
    println!("{:#?}", tim);
}
