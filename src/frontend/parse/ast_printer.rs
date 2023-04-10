use crate::frontend::lex::token::Literal;

use super::expression::*;

#[allow(dead_code)]
pub fn print(expr: &Expression) -> String {
    match expr {
        Expression::Binary {
            left,
            operator,
            right,
        } => parenthesise(&operator.lexeme, vec![left, right]),
        Expression::Ternary {
            condition,
            then_branch,
            else_branch,
        } => parenthesise("ternary", vec![condition, then_branch, else_branch]),
        Expression::Grouping(expr) => parenthesise("group", vec![expr]),
        Expression::Literal(expr) => match expr.as_ref() {
            Some(Literal::Identifier(id)) => id.clone(),
            Some(Literal::String(string)) => string.clone(),
            Some(Literal::Number(number)) => number.to_string(),
            Some(Literal::Boolean(boolean)) => boolean.to_string(),
            None => "nil".to_string(),
        },
        Expression::Unary { operator, right } => parenthesise(&operator.lexeme, vec![right]),
    }
}

fn parenthesise(name: &str, exprs: Vec<&Expression>) -> String {
    let mut result = String::new();
    result.push('(');
    result.push_str(name);
    for expr in exprs {
        result.push(' ');
        result.push_str(&print(expr));
    }
    result.push(')');
    result
}

#[cfg(test)]
mod test {
    use crate::frontend::lex::token::{Literal, Token, TokenType};

    use super::*;

    #[test]
    fn test_astprinter_print() {
        // Expression for -123 * (45.67)
        let expr = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line_number: 1,
                },
                right: Box::new(Expression::Literal(Some(Literal::Number(123.0)))),
            }),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line_number: 1,
            },
            right: Box::new(Expression::Grouping(Box::new(Expression::Literal(Some(
                Literal::Number(45.67),
            ))))),
        };
        let result = print(&expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
