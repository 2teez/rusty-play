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
}
