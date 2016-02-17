#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64)
}

pub type Arguments = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Call(Operator, Arguments),
}
