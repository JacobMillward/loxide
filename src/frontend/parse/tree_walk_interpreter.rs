use crate::frontend::lex::token::{Literal, Token, TokenType};

use super::expression::*;

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub token: Option<Token>,
}

impl RuntimeError {
    pub fn new(message: String) -> Result<Option<Literal>, Self> {
        Err(Self {
            message,
            token: None,
        })
    }

    pub fn with_token(message: String, token: Token) -> Result<Option<Literal>, Self> {
        Err(Self {
            message,
            token: Some(token),
        })
    }

    pub fn operands_must_be_numbers(operator: Token) -> Result<Option<Literal>, Self> {
        Self::with_token("Operands must be numbers.".to_string(), operator)
    }
}

pub fn interpret(expr: &Expression) -> Result<Option<Literal>, RuntimeError> {
    evaluate_expression(expr)
}

fn evaluate_expression(expr: &Expression) -> Result<Option<Literal>, RuntimeError> {
    match expr {
        Expression::Binary { .. } => evaluate_binary(expr),
        Expression::Grouping(_) => evaluate_grouping(expr),
        Expression::Unary { .. } => evaluate_unary(expr),
        Expression::Literal(literal) => Ok(literal.clone()),
        Expression::Ternary {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition = evaluate_expression(condition)?;

            if is_truthy(&condition) {
                evaluate_expression(then_branch)
            } else {
                evaluate_expression(else_branch)
            }
        }
    }
}

fn evaluate_grouping(group: &Expression) -> Result<Option<Literal>, RuntimeError> {
    match group {
        Expression::Grouping(expr) => evaluate_expression(expr),
        _ => RuntimeError::new(format!(
            "Unexpected expression, expected Grouping {:?}",
            group
        )),
    }
}

fn evaluate_binary(binary: &Expression) -> Result<Option<Literal>, RuntimeError> {
    match binary {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(left)?;
            let right = evaluate_expression(right)?;

            match operator.token_type {
                TokenType::Minus => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Number(l - r)))
                    }
                    _ => RuntimeError::operands_must_be_numbers(operator.clone()),
                },

                TokenType::Plus => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Number(l + r)))
                    }

                    (Some(Literal::String(l)), r) => Ok(Some(Literal::String(format!(
                        "{}{}",
                        l,
                        match r {
                            Some(r) => r.to_string(),
                            None => "nil".to_string(),
                        }
                    )))),

                    (l, Some(Literal::String(r))) => Ok(Some(Literal::String(format!(
                        "{}{}",
                        match l {
                            Some(l) => l.to_string(),
                            None => "nil".to_string(),
                        },
                        r
                    )))),

                    _ => RuntimeError::with_token(
                        "operands must be numbers or strings.".to_string(),
                        operator.clone(),
                    ),
                },

                TokenType::Slash => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        if r == 0.0 {
                            return RuntimeError::with_token(
                                "Division by zero.".to_string(),
                                operator.clone(),
                            );
                        }

                        Ok(Some(Literal::Number(l / r)))
                    }
                    _ => RuntimeError::operands_must_be_numbers(operator.clone()),
                },

                TokenType::Star => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Number(l * r)))
                    }
                    _ => RuntimeError::operands_must_be_numbers(operator.clone()),
                },

                TokenType::Greater => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Boolean(l > r)))
                    }
                    _ => Ok(Some(Literal::Boolean(false))),
                },

                TokenType::GreaterEqual => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Boolean(l >= r)))
                    }
                    _ => Ok(Some(Literal::Boolean(false))),
                },

                TokenType::Less => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Boolean(l < r)))
                    }
                    _ => Ok(Some(Literal::Boolean(false))),
                },

                TokenType::LessEqual => match (left, right) {
                    (Some(Literal::Number(l)), Some(Literal::Number(r))) => {
                        Ok(Some(Literal::Boolean(l <= r)))
                    }
                    _ => Ok(Some(Literal::Boolean(false))),
                },

                TokenType::BangEqual => Ok(Some(Literal::Boolean(!evaluate_equal(&left, &right)))),
                TokenType::EqualEqual => Ok(Some(Literal::Boolean(evaluate_equal(&left, &right)))),

                _ => RuntimeError::with_token("Unexpected operator".to_string(), operator.clone()),
            }
        }
        _ => RuntimeError::new("Unexpected expression, expected Binary".to_string()),
    }
}

fn evaluate_unary(unary: &Expression) -> Result<Option<Literal>, RuntimeError> {
    match unary {
        Expression::Unary { operator, right } => {
            let right = evaluate_expression(right)?;

            match operator.token_type {
                TokenType::Minus => match right {
                    Some(Literal::Number(n)) => Ok(Some(Literal::Number(-n))),
                    _ => RuntimeError::operands_must_be_numbers(operator.clone()),
                },

                TokenType::Bang => Ok(Some(Literal::Boolean(!is_truthy(&right)))),

                _ => RuntimeError::with_token("Unexpected operator".to_string(), operator.clone()),
            }
        }
        _ => RuntimeError::new("Unexpected expression, expected Unary".to_string()),
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

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

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

        let result = interpret(&expr);
        assert_eq!(result, Ok(Some(Literal::Number(-1.0))));
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

        assert_eq!(interpret(&expr), Ok(Some(expected)));
    }

    #[rstest]
    #[case::plus_number(Literal::Number(1.0), Literal::Number(2.0), Literal::Number(3.0))]
    #[case::plus_string(Literal::String("hello".to_string()), Literal::String("world".to_string()), Literal::String("helloworld".to_string()))]
    #[case::plus_string_number(Literal::String("hello".to_string()), Literal::Number(1.0), Literal::String("hello1".to_string()))]
    #[case::plus_number_string(Literal::Number(1.0), Literal::String("hello".to_string()), Literal::String("1hello".to_string()))]
    #[case::plus_string_empty(Literal::String("hello".to_string()), Literal::String("".to_string()), Literal::String("hello".to_string()))]
    #[case::plus_string_boolean(Literal::String("hello".to_string()), Literal::Boolean(true), Literal::String("hellotrue".to_string()))]
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

        assert_eq!(interpret(&expr), Ok(Some(expected)));
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

        assert_eq!(interpret(&expr), Ok(Some(expected)));
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

        assert_eq!(interpret(&expr), Ok(Some(expected)));
    }

    #[rstest]
    #[case::greater_string(
        TokenType::Greater,
        Literal::String("hello".to_string()),
        Literal::String("world".to_string())
    )]
    #[case::greater_boolean(TokenType::Greater, Literal::Boolean(true), Literal::Boolean(false))]
    #[case::greater_equal_string(
        TokenType::LessEqual,
        Literal::String("hello".to_string()),
        Literal::String("world".to_string())
    )]
    #[case::greater_equal_boolean(
        TokenType::LessEqual,
        Literal::Boolean(true),
        Literal::Boolean(false)
    )]
    #[case::less_string(
        TokenType::Less,
        Literal::String("hello".to_string()),
        Literal::String("world".to_string())
    )]
    #[case::less_boolean(TokenType::Less, Literal::Boolean(true), Literal::Boolean(false))]
    #[case::less_equal_string(
        TokenType::LessEqual,
        Literal::String("hello".to_string()),
        Literal::String("world".to_string())
    )]
    #[case::less_equal_boolean(
        TokenType::LessEqual,
        Literal::Boolean(true),
        Literal::Boolean(false)
    )]
    fn test_binary_comparison_non_numbers(
        #[case] operator: TokenType,
        #[case] left: Literal,
        #[case] right: Literal,
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

        assert_eq!(interpret(&expr), Ok(Some(Literal::Boolean(false))));
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

        assert_eq!(interpret(&expr), Ok(Some(Literal::Boolean(true))));
    }

    #[test]
    fn test_divide_by_zero() {
        let operator = Token {
            token_type: TokenType::Slash,
            lexeme: "/".to_string(),
            literal: None,
            line_number: 0,
        };

        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Some(Literal::Number(1.0)))),
            operator: operator.clone(),
            right: Box::new(Expression::Literal(Some(Literal::Number(0.0)))),
        };

        assert_eq!(
            interpret(&expr),
            RuntimeError::with_token("Division by zero.".to_string(), operator)
        );
    }

    #[test]
    fn test_grouping() {
        let expr = Expression::Grouping(Box::new(Expression::Literal(Some(Literal::Number(1.0)))));

        assert_eq!(interpret(&expr), Ok(Some(Literal::Number(1.0))));
    }
}
