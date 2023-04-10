use crate::frontend::lex::token::{Literal, Token};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Option<Literal>),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}
