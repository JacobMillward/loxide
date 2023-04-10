use crate::frontend::lex::token::{Token, TokenLiteral};

#[allow(dead_code)]
pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct Grouping {
    pub expression: Box<Expression>,
}

pub struct Literal {
    pub value: Option<TokenLiteral>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expression>,
}
