pub trait Eval {
    fn eval(&self) -> i64;
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operator {
    fn eval_op(&self, args: &Vec<Expression>) -> i64 {
        match *self {
            Operator::Plus => args.iter().fold(0, |acc, x: &Expression| acc + x.eval()),
            Operator::Minus => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(), |acc, x: &Expression| acc - x.eval())
            },
            Operator::Times => args.iter().fold(1, |acc, x: &Expression| acc * x.eval()),
            Operator::Divide => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(), |acc, x: &Expression| acc / x.eval())
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

impl Eval for Literal {
    fn eval(&self) -> i64 {
        match *self {
            Literal::Integer(int) => int,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Call(Operator, Vec<Expression>),
}

impl Eval for Expression {
    fn eval(&self) -> i64 {
        match *self {
            Expression::Literal(ref lit) => lit.eval(),
            Expression::Call(ref op, ref args) => op.eval_op(args),
        }
    }
}
