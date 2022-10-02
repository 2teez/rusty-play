fn main() {
    let mut n = 0;
    while n < 11 {
        println!("{} while loop..", n);
        n += 1;
    }

    while n >= 0 {
        println!("{} while loop rewind..", n);
        n -= 1;
    }

    // using a for loop
    for n in 1..11 {
        println!("Double {} = {}", n, n * n);
    }
}
