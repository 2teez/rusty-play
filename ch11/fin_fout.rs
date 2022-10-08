#![allow(dead_code, unused)]

use std::env::{args, vars};

fn main() {
    println!("[{:?}] [{:?}]", args(), vars());

    // getting the cli arguments in rust
    let cli_arguments = args();
    for arg in cli_arguments {
        println!("[{}]", arg);
    }

    // display all the environment variable in rust
    println!("{:?} {:?}", vars(), std::env::var("first"));
    for v in vars() {
        println!("{:?}", v);
    }
}
