fn main() {
    let mut a: [i32; 10] = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double(&mut a);
    println!("{:?}", a);

    let a = f('a', 32.145, 90.2);
    let b = f('c', 23, 09);

    println!("{} {}", a, b);
}

fn double(a: &mut [i32; 10]) {
    for n in 0..a.len() {
        a[n] *= 2;
    }
}

// using a generic function
fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' {
        num1
    } else {
        num2
    }
}
