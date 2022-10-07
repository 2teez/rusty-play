#![allow(unused, dead_code)]

fn main() {
    let nums = [44, 42, 12, 10];

    for n in nums {
        println!("{}", n);
    }

    let mut n = nums.len();
    let mut iter = nums.iter();
    loop {
        // will not work
        // array/vectors are not iterators
        //let item = nums.next();
        let item = iter.next();
        if let Some(num) = item {
            println!("{}", num);
        }
        if n == 0 {
            break;
        }
        n -= 1;
    }
    // OR
    let mut iter2 = nums.iter();
    println!("Another Way:");
    loop {
        match iter2.next() {
            Some(n) => println!("{}", n),
            None => {
                println!("{}", "");
                break;
            }
        }
    }

    // using a mutable vector to be changed
    let mut vec = vec![44, 42, 12, 10];
    for item in vec.iter_mut() {
        *item += 1;
    }
    println!("{:?}", vec);
}
