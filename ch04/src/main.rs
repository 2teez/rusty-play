
fn main() {
    let names = vec!["tim", "tolu", "melchi", "temi"];
    for name in names {
        println!("{}", name);
    }

    let mut nnames = vec!["tim".to_string(), "tolu".to_string()];
    for name in nnames.iter_mut() {
        *name = name.to_owned() + &" adigun".to_string();
    }
    println!("{:?}", nnames);

}
