use crate::frontend::lex::token::{Literal, TokenType};

use super::expression::*;

pub fn interpret(expr: &Expression) -> Option<Literal> {
    evaluate_expression(expr)
}

fn evaluate_expression(expr: &Expression) -> Option<Literal> {
    match expr {
        Expression::Binary { .. } => evaluate_binary(expr),
        Expression::Grouping(_) => evaluate_grouping(expr),
        Expression::Unary { .. } => evaluate_unary(expr),
        Expression::Literal(literal) => literal.clone(),
        _ => panic!("Unexpected expression {:?}", expr),
    }
}

fn evaluate_grouping(group: &Expression) -> Option<Literal> {
    match group {
        Expression::Grouping(expr) => evaluate_expression(expr),
        _ => panic!("Unexpected expression, expected Grouping {:?}", group),
    }
}

fn evaluate_binary(binary: &Expression) -> Option<Literal> {
    match binary {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(left);
            let right = evaluate_expression(right);

            match operator.token_type {
                TokenType::Minus => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Number(l - r))
                    }
                    _ => panic!("Binary Minus - expects two numbers"),
                },

                TokenType::Plus => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Number(l + r))
                    }
                    (Some(Literal::String(l)), Some(Literal::String(r))) => {
                        Some(Literal::String(format!("{}{}", l, r)))
                    }
                    _ => panic!("Binary Plus + expects two numbers or two strings"),
                },

                TokenType::Slash => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Number(l / r))
                    }
                    _ => panic!("Binary Slash / expects two numbers"),
                },

                TokenType::Star => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Number(l * r))
                    }
                    _ => panic!("Binary Star * expects two numbers"),
                },

                TokenType::Greater => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Boolean(l > r))
                    }
                    _ => panic!("Binary Greater > expects two numbers"),
                },

                TokenType::GreaterEqual => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Boolean(l >= r))
                    }
                    _ => panic!("Binary GreaterEqual >= expects two numbers"),
                },

                TokenType::Less => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Boolean(l < r))
                    }
                    _ => panic!("Binary Less < expects two numbers"),
                },

                TokenType::LessEqual => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Some(Literal::Boolean(l <= r))
                    }
                    _ => panic!("Binary LessEqual <= expects two numbers"),
                },

                TokenType::BangEqual => Some(Literal::Boolean(!evaluate_equal(&left, &right))),
                TokenType::EqualEqual => Some(Literal::Boolean(evaluate_equal(&left, &right))),

                _ => panic!("Unknown binary operator {:?}", operator),
            }
        }
        _ => panic!("Unexpected expression, expected Binary {:?}", binary),
    }
}

fn evaluate_unary(unary: &Expression) -> Option<Literal> {
    match unary {
        Expression::Unary { operator, right } => {
            let right = evaluate_expression(right);
            match operator.token_type {
                TokenType::Minus => match right {
                    Some(Literal::Number(n)) => Some(Literal::Number(-n)),
                    _ => panic!("Unary Minus - expects a number"),
                },
                TokenType::Bang => Some(Literal::Boolean(!is_truthy(&right))),
                _ => panic!("Unknown unary operator {:?}", operator),
            }
        }
        _ => panic!("Unexpected expression, expected Unary {:?}", unary),
    }
}

fn is_truthy(literal: &Option<Literal>) -> bool {
    match literal {
        Some(Literal::Boolean(b)) => *b,
        None => false,
        _ => true,
    }
}

fn evaluate_equal(left: &Option<Literal>, right: &Option<Literal>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,

        (Some(Literal::Number(l)), Some(Literal::Number(r))) => l == r,
        (Some(Literal::Number(_)), Some(_)) => false,

        (Some(Literal::String(l)), Some(Literal::String(r))) => l == r,
        (Some(Literal::String(_)), Some(_)) => false,

        (Some(Literal::Boolean(l)), Some(Literal::Boolean(r))) => l == r,
        (Some(Literal::Boolean(_)), Some(_)) => false,

        (Some(Literal::Identifier(l)), Some(Literal::Identifier(r))) => l == r,
        (Some(Literal::Identifier(_)), Some(_)) => false,
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Number(l), Literal::Number(r)) => l == r,
            (Literal::String(l), Literal::String(r)) => l == r,
            (Literal::Boolean(l), Literal::Boolean(r)) => l == r,
            (Literal::Identifier(l), Literal::Identifier(r)) => l == r,

            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use crate::frontend::lex::token::Token;

    #[test]
    fn test_literal_equality() {
        assert_eq!(Literal::Number(1.0), Literal::Number(1.0));
        assert_ne!(Literal::Number(1.0), Literal::Number(2.0));
        assert_eq!(
            Literal::String("hello".to_string()),
            Literal::String("hello".to_string())
        );
        assert_ne!(
            Literal::String("hello".to_string()),
            Literal::String("world".to_string())
        );
        assert_eq!(Literal::Boolean(true), Literal::Boolean(true));
        assert_ne!(Literal::Boolean(true), Literal::Boolean(false));
    }

    #[rstest]
    #[case::boolean_true(Literal::Boolean(true), true)]
    #[case::boolean_false(Literal::Boolean(false), false)]
    #[case::number(Literal::Number(1.0), true)]
    #[case::string(Literal::String("hello".to_string()), true)]
    #[case::string_false(Literal::String("false".to_string()), true)]
    #[case::string_true(Literal::String("true".to_string()), true)]
    #[case::string_empty(Literal::String("".to_string()), true)]
    #[case::identifier(Literal::Identifier("foo".to_string()), true)]
    fn test_literal_truthiness(#[case] literal: Literal, #[case] expected: bool) {
        assert_eq!(is_truthy(&Some(literal)), expected);
    }

    #[test]
    fn test_unary_minus() {
        let expr = Expression::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(Literal::Number(1.0)))),
        };

        assert_eq!(interpret(&expr), Some(Literal::Number(-1.0)));
    }

    #[rstest]
    #[case::boolean_true(Literal::Boolean(true), Literal::Boolean(false))]
    #[case::boolean_false(Literal::Boolean(false), Literal::Boolean(true))]
    #[case::number(Literal::Number(1.0), Literal::Boolean(false))]
    #[case::string(Literal::String("hello".to_string()), Literal::Boolean(false))]
    #[case::string_false(Literal::String("false".to_string()), Literal::Boolean(false))]
    #[case::string_true(Literal::String("true".to_string()), Literal::Boolean(false))]
    #[case::string_empty(Literal::String("".to_string()), Literal::Boolean(false))]
    #[case::identifier(Literal::Identifier("foo".to_string()), Literal::Boolean(false))]
    fn test_unary_bang(#[case] input: Literal, #[case] expected: Literal) {
        let expr = Expression::Unary {
            operator: Token {
                token_type: TokenType::Bang,
                lexeme: "!".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(input))),
        };

        assert_eq!(interpret(&expr), Some(expected));
    }

    #[rstest]
    #[case::plus_number(Literal::Number(1.0), Literal::Number(2.0), Literal::Number(3.0))]
    #[case::plus_string(Literal::String("hello".to_string()), Literal::String("world".to_string()), Literal::String("helloworld".to_string()))]
    fn test_binary_plus(#[case] left: Literal, #[case] right: Literal, #[case] expected: Literal) {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Some(left))),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(right))),
        };

        assert_eq!(interpret(&expr), Some(expected));
    }

    #[rstest]
    #[case::minus(
        TokenType::Minus,
        Literal::Number(3.0),
        Literal::Number(2.0),
        Literal::Number(1.0)
    )]
    #[case::divide(
        TokenType::Slash,
        Literal::Number(6.0),
        Literal::Number(3.0),
        Literal::Number(2.0)
    )]
    #[case::multiply(
        TokenType::Star,
        Literal::Number(2.0),
        Literal::Number(1.0),
        Literal::Number(2.0)
    )]
    fn test_binary_arithmetic(
        #[case] operator: TokenType,
        #[case] left: Literal,
        #[case] right: Literal,
        #[case] expected: Literal,
    ) {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Some(left))),
            operator: Token {
                lexeme: match operator {
                    TokenType::Minus => "-".to_string(),
                    TokenType::Slash => "/".to_string(),
                    TokenType::Star => "*".to_string(),
                    _ => panic!("Unexpected operator {:?}", operator),
                },
                token_type: operator,
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(right))),
        };

        assert_eq!(interpret(&expr), Some(expected));
    }

    #[rstest]
    #[case::greater(
        TokenType::Greater,
        Literal::Number(2.0),
        Literal::Number(1.0),
        Literal::Boolean(true)
    )]
    #[case::greater_equal(
        TokenType::GreaterEqual,
        Literal::Number(2.0),
        Literal::Number(2.0),
        Literal::Boolean(true)
    )]
    #[case::less(
        TokenType::Less,
        Literal::Number(1.0),
        Literal::Number(2.0),
        Literal::Boolean(true)
    )]
    #[case::less_equal(
        TokenType::LessEqual,
        Literal::Number(2.0),
        Literal::Number(2.0),
        Literal::Boolean(true)
    )]

    fn test_binary_comparison(
        #[case] operator: TokenType,
        #[case] left: Literal,
        #[case] right: Literal,
        #[case] expected: Literal,
    ) {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Some(left))),
            operator: Token {
                lexeme: match operator {
                    TokenType::Greater => ">".to_string(),
                    TokenType::GreaterEqual => ">=".to_string(),
                    TokenType::Less => "<".to_string(),
                    TokenType::LessEqual => "<=".to_string(),
                    _ => panic!("Unexpected operator {:?}", operator),
                },
                token_type: operator,
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(right))),
        };

        assert_eq!(interpret(&expr), Some(expected));
    }

    #[rstest]
    #[case::equal_number(TokenType::EqualEqual, Literal::Number(1.0), Literal::Number(1.0))]
    #[case::bang_equal_number(TokenType::BangEqual, Literal::Number(1.0), Literal::Number(2.0))]
    #[case::equal_string(
        TokenType::EqualEqual,
        Literal::String("hello".to_string()),
        Literal::String("hello".to_string()),
    )]
    #[case::bang_equal_string(
        TokenType::BangEqual,
        Literal::String("hello".to_string()),
        Literal::String("hello world".to_string()),
    )]
    #[case::equal_boolean(TokenType::EqualEqual, Literal::Boolean(true), Literal::Boolean(true))]
    #[case::bang_equal_boolean(
        TokenType::BangEqual,
        Literal::Boolean(true),
        Literal::Boolean(false)
    )]
    fn test_binary_equality(
        #[case] operator: TokenType,
        #[case] left: Literal,
        #[case] right: Literal,
    ) {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Some(left))),
            operator: Token {
                lexeme: match operator {
                    TokenType::EqualEqual => "==".to_string(),
                    TokenType::BangEqual => "!=".to_string(),
                    _ => panic!("Unexpected operator {:?}", operator),
                },
                token_type: operator,
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expression::Literal(Some(right))),
        };

        assert_eq!(interpret(&expr), Some(Literal::Boolean(true)));
    }

    #[test]
    fn test_grouping() {
        let expr = Expression::Grouping(Box::new(Expression::Literal(Some(Literal::Number(1.0)))));

        assert_eq!(interpret(&expr), Some(Literal::Number(1.0)));
    }
}
