// a struct like tuple
// can be used like namedtuple in python
#[derive(Debug)]
struct Persons<'a>(&'a str, u8, char);

fn main() {
    let tomi = ("tomi", 15, "female");
    let kunle: (&str, u8, &str) = ("kunle", 18, "male");

    println!("{:#?}, {:#?}", tomi, kunle);
    println!("Age: {}", kunle.1);

    let tomi = Persons("tomi", 34, 'F');
    println!("{:#?}", tomi);
    println!("Name: {}, Gender: {}", tomi.0, tomi.2);
}
