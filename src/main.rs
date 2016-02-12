#[macro_use]
extern crate nom;

use std::io;
use std::io::Write;

use std::str::{FromStr, from_utf8_unchecked};

pub fn to_string(s: &[u8]) -> &str {
    unsafe { from_utf8_unchecked(s) }
}
pub fn to_i32(s: &str) -> i32 {
    FromStr::from_str(s).unwrap()
}
pub fn to_u32(s: &str) -> u32 {
    FromStr::from_str(s).unwrap()
}

pub fn buf_to_u32(s: &[u8]) -> u32 {
    to_u32(to_string(s))
}
pub fn buf_to_i32(s: &[u8]) -> i32 {
    to_i32(to_string(s))
}

pub fn vec_to_i32(v: Vec<char>) -> i32 {
    to_i32(v.into_iter().collect::<String>().as_ref())
}

named!(sign <&[u8], i32>, map!(tag!("-"), |_| -1 ));
named!(integer  <&[u8], i32>, map!(many1!(one_of!(b"0123456789")), vec_to_i32));
named!(pub number <&[u8], i32>, chain!(
        pref: opt!(sign) ~
        y:    integer
        ,
        || {
            pref.unwrap_or(1) * y
        }));

fn parse(s: &mut String) {
    let num = number(s.as_bytes());
    println!("{:?}", num);
}

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
            Ok(_) => parse(&mut input),
            Err(error) => println!("error {}", error),
        }
    }
}
