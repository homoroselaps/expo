use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub trait Eval {
    fn eval(&self) -> ExpoResult;
}

#[derive(Debug)]
pub enum ExpoResult {
    Value(i64),
    Error(Error),
}

impl ExpoResult {
    fn map<F>(self, fun: F, res: ExpoResult) -> ExpoResult
        where F: Fn(i64, i64) -> i64 {

        match self {
            ExpoResult::Value(int) => {
                match res {
                    ExpoResult::Value(num) => {
                        ExpoResult::Value(fun(int, num))
                    },
                    ExpoResult::Error(error) => {
                        ExpoResult::Error(error)
                    },
                }
            },
            ExpoResult::Error(error) => {
                ExpoResult::Error(error)
            },
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Unknown,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operator {
    fn eval_op(&self, args: &Vec<Expression>) -> ExpoResult {
        match *self {
            Operator::Plus => args.iter().fold(ExpoResult::Value(0),
                |acc, x: &Expression| {
                    acc.map(Add::add, x.eval())
            }),
            Operator::Times => args.iter().fold(ExpoResult::Value(1),
                |acc, x: &Expression| {
                    acc.map(Mul::mul, x.eval())
            }),
            Operator::Minus => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(),
                    |acc, x: &Expression| {
                        acc.map(Sub::sub, x.eval())
                    }
                )
            },
            Operator::Divide => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(),
                    |acc, x: &Expression| {
                        acc.map(Div::div, x.eval())
                    }
                )
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

impl Eval for Literal {
    fn eval(&self) -> ExpoResult {
        match *self {
            Literal::Integer(int) => ExpoResult::Value(int),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Call(Operator, Vec<Expression>),
}

impl Eval for Expression {
    fn eval(&self) -> ExpoResult {
        match *self {
            Expression::Literal(ref lit) => lit.eval(),
            Expression::Call(ref op, ref args) => op.eval_op(args),
        }
    }
}
