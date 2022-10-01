#[allow(dead_code, unused_mut)]

fn main() {
    // useless `mut` used
    // silence the warning from the
    // compiler with the #[allow(unused_mut)]
    let mut number = 12;

    println!("{}", number);
}
