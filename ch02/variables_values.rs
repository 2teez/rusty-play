#[allow(dead_code, unused_mut)]

fn main() {
    // useless `mut` used
    // silence the warning from the
    // compiler with the #[allow(unused_mut)]
    let mut number = 12;

    println!("{}", number);

    let number2;
    let number3;

    number2 = 34;
    number3 = number2;

    println!("{} {}", number2, number3);

    // Boolean variables and values
    let mut truth = true;
    let mut failsy = false;

    println!("{}, {}, {}", truth, failsy, -50 < 6);

    truth = 5 < 2;
    failsy = 3 != 5;

    println!("{} {}", truth, failsy);

    let mut _n = 1;
    _n = 2;
    let _n = 3.14;
    //_n = 5.9;  // can't assign twice to immutable variable
    println!("{}", _n);

    // function usage -- Standard Library Usage:
    println!("{} {}", str::len("abcd"), "abcd".len());
}
