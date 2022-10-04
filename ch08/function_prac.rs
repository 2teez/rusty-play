fn main() {
    let mut a: [i32; 10] = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double(&mut a);
    println!("{:?}", a);

    let a = f('a', 32.145, 90.2);
    let b = f('c', 23, 09);

    println!("{} {}", a, b);

    let a = ["tim", "ermys"];
    let b = true;

    println!("{:?}", swap(a, b));

    // taking a value from a Option enum
    // by using `if-let`
    let a: Option<u32> = Some(45);
    let mut value: u32 = 0;
    if let Some(num) = a {
        value = num;
    }
    println!("The Value is {}", value);

    println!("{:?}", divider(7.3, 1.5));
    println!("{:?}", divider(7.3, 0.));

    let value = divider(8.21, 0.);
    println!(
        "{}",
        match value {
            Ok(num) => num.to_string(),
            Err(s) => s,
        }
    );
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

// doing swap in rust
// using generic to do so
fn swap<T1, T2>(val1: T1, val2: T2) -> (T2, T1) {
    (val2, val1)
}

fn divider(nume: f64, deno: f64) -> Result<f64, String> {
    if deno == 0. {
        return Err("Can't divide with Zero".to_string());
    }
    Ok(nume / deno)
}
