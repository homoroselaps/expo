use nom::IResult;

use utils::vec_to_i64;
use ast::*;

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
            Literal::Integer(pref.unwrap_or(1) * y.eval())
        }
));

named!(arguments <&[u8], Vec<Expression> >, many1!(
    chain!(
        tag!(" ") ~
        exp: expression,
        || { exp }
    )
));

named!(pub expression <&[u8], Expression>, alt!(
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

pub fn parse(s: &mut String) {
    let expr = expression(s.as_bytes());
    if let IResult::Done(i, output) = expr {
        println!("{:?}", output.eval());
    }
    else {
        println!("error while parsing: {:?}", expr);
    }
}
