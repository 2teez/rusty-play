#![allow(dead_code, unused)]

use std::env::{args, vars};
use std::io::{stdin, stdout, Write};

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

    // read a line from stdin
    stdout().write("Enter your name: ".as_bytes()).unwrap();
    let mut line = String::new();
    let input = stdin().read_line(&mut line);
    if input.is_ok() {
        println!("{}", line);
    }
}
