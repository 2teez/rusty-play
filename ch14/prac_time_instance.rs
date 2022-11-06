#![allow(dead_code)]

use std::time::Instant;

fn main() {
    done(
        "checking timing for making a million integer on vector..",
        &|| {
            let vec: Vec<i32> = Vec::<i32>::with_capacity(1_000_000);
            println!("{:?}", vec);
        },
    );
    done(
        "checking timing for push a million integer on vector..",
        &|| {
            let mut vec: Vec<i32> = vec![];
            for data in 0..1_000_000 {
                vec.push(data);
            }
        },
    );
}

fn done(msg: &str, f: &dyn Fn() -> ()) {
    println!("{}.", msg);
    let t = Instant::now();
    f();
    let t1 = t.elapsed();
    println!("[Done].. {:?}", t1);
}
