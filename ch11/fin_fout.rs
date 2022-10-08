#![allow(dead_code, unused)]

use std::env::{args, vars};
use std::io::{self, stdin, stdout, BufRead, Read, Write};

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
    //stdout().write("Enter your name: ".as_bytes()).unwrap();
    let mut line = String::new();
    //let input = stdin().read_line(&mut line);
    let input = get_user_input("Enter your name: ");
    if input.is_ok() {
        println!("{}", input.unwrap());
    }

    // print out an input
    print!("{}", "Enter> ");
    io::stdout().flush();
    stdin().read_line(&mut line).unwrap();
    println!("{}", line);

    // read out a whole file
    print!("{}", "Enter a filename: ");
    io::stdout().flush();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    filename = filename.trim().to_string();
    let file = std::fs::File::open(filename).unwrap();
    let mut buf = io::BufReader::new(file);
    for line in buf.lines() {
        println!(
            "{}",
            match line {
                Ok(line) => line,
                Err(_) => "error...".to_string(),
            }
        );
    }

    let name = get_user_input("Enter a number: ");
    if name.is_ok() {
        println!("{}", name.unwrap());
    }
}

fn get_user_input<'a>(msg: &'a str) -> Result<String, std::io::Error> {
    print!("{}", msg);
    io::stdout().flush();

    // line to use
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}
