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
    //
    // Adapter
    let nums = [66, -8, 43, 19, 0, -31];
    println!("{:?}", nums.iter().filter(|n| *n < &0).collect::<Vec<_>>());
    println!("{:?}", nums.iter().map(|n| n * n).collect::<Vec<_>>());
    println!("{:?}", nums.iter().enumerate().collect::<Vec<_>>());
}

fn prnt<'a>(msg: &'a str, f: &dyn Fn() -> ()) -> () {
    println!("{}", msg);
    println!("{:?}", f())
}
