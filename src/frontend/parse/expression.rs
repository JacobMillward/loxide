use crate::frontend::lex::token::{Literal, Token};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Option<Literal>),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}
