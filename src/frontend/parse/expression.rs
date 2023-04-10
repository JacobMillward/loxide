use crate::frontend::lex::token::{Token, TokenLiteral};

pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &mut dyn ExpressionVisitor<T>) -> T {
        match self {
            Expression::Binary(expr) => visitor.visit_binary_expr(expr),
            Expression::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expression::Literal(expr) => visitor.visit_literal_expr(expr),
            Expression::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
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

pub trait ExpressionVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
}
