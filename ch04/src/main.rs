
fn main() {
    let names = vec!["tim", "tolu", "melchi", "temi"];
    for name in names {
        println!("{}", name);
    }

    let mut nnames = vec!["tim".to_string(), "tolu".to_string()];
    for name in nnames.iter_mut() {
        *name = name.to_string() + " adigun";
    }
    println!("{:?}", nnames);
    nnames[0].make_ascii_uppercase();
    nnames.iter_mut().for_each(|s| {s.make_ascii_uppercase()});
    println!("{:?}", nnames);

}
