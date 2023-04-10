use super::expression::Expression;
use crate::frontend::lex::token::{Literal, Token, TokenType};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

type ParseResult<T> = Result<T, ParseError>;

/**
 * Implements a recursive descent parser for the formal grammar:
 * expression   => comma ;
 * comma        => equality ( "," equality )* ;
 * ternary      => equality ( "?" expression ":" expression )? ;
 * equality     => comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison   => term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term         => factor ( ( "-" | "+" ) factor )* ;
 * factor       => unary ( ( "/" | "*" ) unary )* ;
 * unary        => ( "!" | "-" ) unary
 *              | primary ;
 * primary      => NUMBER | STRING | "false" | "true" | "nil"
 *              | "(" expression ")" ;
*/
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Expression> {
        self.expression()
    }

    fn create_left_associative_binary_expression(
        &mut self,
        token_types: Vec<TokenType>,
        next: fn(&mut Self) -> ParseResult<Expression>,
    ) -> ParseResult<Expression> {
        let mut expr = next(self)?;

        while self.next_matches(&token_types) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.get_previous().clone(),
                right: Box::new(next(self)?),
            };
        }

        Ok(expr)
    }

    fn expression(&mut self) -> ParseResult<Expression> {
        self.comma()
    }

    fn comma(&mut self) -> ParseResult<Expression> {
        self.create_left_associative_binary_expression(vec![TokenType::Comma], Self::ternary)
    }

    fn ternary(&mut self) -> ParseResult<Expression> {
        let mut expr = self.equality()?;

        if self.next_matches(&vec![TokenType::QuestionMark]) {
            let then_branch = self.expression()?;
            self.consume(&TokenType::Colon, "Expected ':' after then branch")?;
            let else_branch = self.expression()?;
            expr = Expression::Ternary {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expression> {
        self.create_left_associative_binary_expression(
            vec![TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparison,
        )
    }

    fn comparison(&mut self) -> ParseResult<Expression> {
        self.create_left_associative_binary_expression(
            vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
            Self::term,
        )
    }

    fn term(&mut self) -> ParseResult<Expression> {
        self.create_left_associative_binary_expression(
            vec![TokenType::Minus, TokenType::Plus],
            Self::factor,
        )
    }

    fn factor(&mut self) -> ParseResult<Expression> {
        self.create_left_associative_binary_expression(
            vec![TokenType::Slash, TokenType::Star],
            Self::unary,
        )
    }

    fn unary(&mut self) -> ParseResult<Expression> {
        if self.next_matches(&vec![TokenType::Bang, TokenType::Minus]) {
            Ok(Expression::Unary {
                operator: self.get_previous().clone(),
                right: Box::new(self.unary()?),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ParseResult<Expression> {
        match self.peek().token_type {
            TokenType::False => {
                self.advance();
                Ok(Expression::Literal(Some(Literal::Boolean(false))))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Literal(Some(Literal::Boolean(true))))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expression::Literal(None))
            }
            TokenType::Number => {
                self.advance();
                Ok(Expression::Literal(Some(Literal::Number(
                    self.get_previous().lexeme.parse().unwrap(),
                ))))
            }
            TokenType::String => {
                self.advance();
                Ok(Expression::Literal(Some(Literal::String(
                    self.get_previous().lexeme.clone(),
                ))))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => Err(ParseError {
                token: self.peek().clone(),
                message: "Expect expression.".to_string(),
            }),
        }
    }

    fn next_matches(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check_next(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> ParseResult<()> {
        if self.check_next(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                token: self.peek().clone(),
                message: message.to_string(),
            })
        }
    }

    fn check_next(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.get_previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn get_previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    #[allow(dead_code)]
    /**
     * Synchronise the parser to the next statement.
     * This is used to recover from errors by skipping
     * tokens until we reach a semicolon or a statement
     */
    fn syncronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.get_previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parses_simple_expression() {
        let mut parser = super::Parser::new(vec![
            Token {
                token_type: super::TokenType::Number,
                lexeme: "123".to_string(),
                literal: Some(super::Literal::Number(123.0)),
                line_number: 1,
            },
            Token {
                token_type: super::TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line_number: 1,
            },
        ]);

        let expr = parser.parse().unwrap();

        assert_eq!(
            expr,
            super::Expression::Literal(Some(super::Literal::Number(123.0)))
        );
    }
}
