use crate::frontend::lex::token::TokenLiteral;

use super::expression::*;

#[allow(dead_code)]
pub fn print(expr: &Expression) -> String {
    match expr {
        Expression::Binary(expr) => {
            parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
        }

        Expression::Grouping(expr) => parenthesize("group", vec![&expr.expression]),
        Expression::Literal(expr) => match expr.value.as_ref() {
            Some(TokenLiteral::Identifier(id)) => id.clone(),
            Some(TokenLiteral::String(string)) => string.clone(),
            Some(TokenLiteral::Number(number)) => number.to_string(),
            None => "nil".to_string(),
        },
        Expression::Unary(expr) => parenthesize(&expr.operator.lexeme, vec![&expr.right]),
    }
}

fn parenthesize(name: &str, exprs: Vec<&Expression>) -> String {
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
    use crate::frontend::lex::token::{Token, TokenLiteral, TokenType};

    use super::*;

    #[test]
    fn test_astprinter_print() {
        // Expression for -123 * (45.67)
        let expr = Expression::Binary(Binary {
            left: Box::new(Expression::Unary(Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line_number: 1,
                },
                right: Box::new(Expression::Literal(Literal {
                    value: Some(TokenLiteral::Number(123.0)),
                })),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line_number: 1,
            },
            right: Box::new(Expression::Grouping(Grouping {
                expression: Box::new(Expression::Literal(Literal {
                    value: Some(TokenLiteral::Number(45.67)),
                })),
            })),
        });
        let result = print(&expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
