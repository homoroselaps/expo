extern crate expo;

use std::io;
use std::io::Write;

use expo::parser;

fn main() {
    // Print Version and Exit Information
    println!("Expo Version 0.0.1");
    println!("Press Ctrl+c to Exit");
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        print!("expo> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(_) => parser::parse(&mut input),
            Err(error) => println!("error {}", error),
        }
    }
}
