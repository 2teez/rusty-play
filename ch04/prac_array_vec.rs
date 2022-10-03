#[allow(unused)]
fn main() {
    let x = ["a"];
    //#[allow(unconditional_panic)]
    // code below will panic
    // though rust attribute code above
    // will allow you to compile your code
    // but it would not work
    //let _y = x[1];

    let mut x = ["X", "Y", "Z"];
    //x = ["A", "B"];
    //x = [1, 45, 8];
    println!("{:?}", x);

    let mut names: [&str; 4] = [""; 4];
    names[0] = "timothy";
    names[1] = "tolulope";
    println!("{:?}", names);
    names[2] = "melchizedek";
    names[3] = "temiloluwa";
    println!("{:?}", names);

    let family: [[&str; 2]; 2] = [["tim", "tolu"], ["melchi", "temi"]];
    for index in 0..family.len() {
        println!("{} {:?}", index, family[index]);
    }

    for grp in family {
        let mut names = grp;
        for name in names.iter_mut() {
            // work on String later....
            //*name = &(name.to_owned() + &" adigun".to_owned());
            //name = format!("{} {}", name, "adigun");
            print!("{}, ", name);
        }
    }
}
