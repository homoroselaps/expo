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
pub fn to_i64(s: &str) -> i64 {
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
pub fn vec_to_i64(v: Vec<char>) -> i64 {
    to_i64(v.into_iter().collect::<String>().as_ref())
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}
#[derive(Debug)]
enum Literal {
    Integer(i64)
}
type Arguments = Vec<Expression>;
#[derive(Debug)]
enum Expression {
    Literal(Literal),
    Call(Operator, Arguments),
}

named!(sign <&[u8], i64>, map!(tag!("-"), |_| -1 ));
named!(integer  <&[u8], Literal>, map!(
    many1!(one_of!(b"0123456789")),
    | vector |  Literal::Integer(vec_to_i64(vector))
));
named!(operator <&[u8], Operator>, alt!(
    map!(tag!("+"), |_| Operator::Plus) |
    map!(tag!("-"), |_| Operator::Minus) |
    map!(tag!("*"), |_| Operator::Times) |
    map!(tag!("/"), |_| Operator::Divide)
));
named!(number <&[u8], Literal>, chain!(
        pref: opt!(sign) ~
        y:    integer,
        || {
            let Literal::Integer(num) = y;
            Literal::Integer(pref.unwrap_or(1) * num)
        }
));
named!(arguments <&[u8], Arguments>, many1!(
    chain!(
        tag!(" ") ~
        exp: expression,
        || { exp }
    )
));
named!(expression <&[u8], Expression>, alt!(
    chain!(
        num: number,
        || { Expression::Literal(num) }
    ) |
    chain!(
        open: char!('(') ~
        op: operator ~
        args: arguments ~
        close: char!(')'),
        || { Expression::Call(op, args) }
    )
));

fn parse(s: &mut String) {
    let expr = expression(s.as_bytes());
    println!("{:?}", expr);
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
