#![allow(dead_code, unused)]
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
#[derive(Debug)]
struct PersonTuple<'a>(&'a str, usize, char);

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: usize,
    gender: Gender,
}
fn main() {
    let a = 7;
    let mut a_ref = &a;
    let mut a_box: Box<i32> = Box::new(*a_ref);

    println!("VALUE: {}, REF: {}, BOX: {}", a, a_ref, a_box);

    let c = a + 2;
    a_ref = &c;
    a_box = Box::new(a + 2);
    println!("VALUE: {}, REF: {}, BOX: {}", a, a_ref, a_box);

    // using std::mem::size_of::<Type>
    // or using std::mem::size_of_val(&Type)
    // to get the size of any type
    let tim: Person = Person {
        name: "timothy",
        age: 34,
        gender: Gender::Male,
    };
    println!(
        "{} {} {} {}",
        std::mem::size_of_val(&tim),
        std::mem::size_of::<&Person>(),
        std::mem::size_of::<&PersonTuple>(),
        std::mem::size_of::<&Gender>()
    );
}
