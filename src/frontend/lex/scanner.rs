use std::iter::Peekable;
use std::string::String;

use unicode_segmentation::GraphemeIndices;
use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::lex::token::TokenType;
use crate::frontend::LoxErrorReport;

use super::token::Literal;
use super::token::Token;
use super::token::TokenType::*;

#[derive(Clone, Debug)]
pub enum PossibleToken {
    Ok(Token),
    Err(LoxErrorReport),
}

pub struct Scanner {
    line_number: usize,
    lexeme_start: usize,
    lexeme_current: usize,
    tokens: Vec<PossibleToken>,
}

impl Scanner {
    pub fn scan_tokens(source: &str) -> Vec<PossibleToken> {
        let mut scanner = Scanner {
            line_number: 0,
            lexeme_start: 0,
            lexeme_current: 0,
            tokens: Vec::new(),
        };

        // Get an iterator over the graphemes in the line
        let mut grapheme_iter = UnicodeSegmentation::grapheme_indices(source, true).peekable();

        while let Some((grapheme_idx, g)) = grapheme_iter.next() {
            scanner.lexeme_start = grapheme_idx;
            scanner.lexeme_current = grapheme_idx;

            let mut add_if_next_matches =
                |expected: &str, on_true: TokenType, on_false: TokenType| {
                    if scanner.next_matches(&mut grapheme_iter, expected) {
                        scanner.add_token(on_true, source)
                    } else {
                        scanner.add_token(on_false, source)
                    }
                };

            match g {
                // Single character tokens
                "(" => scanner.add_token(LeftParen, source),
                ")" => scanner.add_token(RightParen, source),
                "{" => scanner.add_token(LeftBrace, source),
                "}" => scanner.add_token(RightBrace, source),
                "," => scanner.add_token(Comma, source),
                "." => scanner.add_token(Dot, source),
                "-" => scanner.add_token(Minus, source),
                "+" => scanner.add_token(Plus, source),
                ";" => scanner.add_token(Semicolon, source),
                "*" => scanner.add_token(Star, source),

                // One or two character tokens
                "!" => add_if_next_matches("=", BangEqual, Bang),
                "=" => add_if_next_matches("=", EqualEqual, Equal),
                "<" => add_if_next_matches("=", LessEqual, Less),
                ">" => add_if_next_matches("=", GreaterEqual, Greater),

                // Comments or division
                "/" => {
                    if scanner.next_matches(&mut grapheme_iter, "/") {
                        while let Some(_) = grapheme_iter.next_if(|(_, g)| *g != "\n") {}
                        scanner.line_number += 1;
                    } else {
                        scanner.add_token(Slash, source)
                    }
                }

                // Ignore whitespace
                " " | "\r" | "\t" => {}

                // Newline
                "\n" => scanner.line_number += 1,

                // String
                "\"" => scanner.parse_string(&mut grapheme_iter, source),

                // Number
                _ if is_digit(g) => scanner.parse_number(&mut grapheme_iter, source),

                // Identifier
                _ if is_alpha(g) => scanner.parse_identifier(&mut grapheme_iter, source),

                // Invalid token
                _ => scanner.tokens.push(PossibleToken::Err(LoxErrorReport::new(
                    scanner.line_number,
                    format!(""),
                    format!(
                        "Invalid token at line {} pos {}: {}",
                        scanner.line_number, grapheme_idx, g
                    ),
                ))),
            }
        }

        scanner.tokens.push(PossibleToken::Ok(Token::new(
            EOF,
            "".to_string(),
            None,
            scanner.line_number,
        )));
        scanner.tokens
    }

    /**
     * Gets the lexeme from the current line
     */
    fn get_lexeme(&self, src: &str) -> String {
        src[self.lexeme_start..self.lexeme_current + 1].to_string()
    }

    /**
     * Adds a token to the list of tokens
     */
    fn add_token(&mut self, token_type: TokenType, src: &str) {
        self.tokens.push(PossibleToken::Ok(Token::new(
            token_type,
            self.get_lexeme(src),
            None,
            self.line_number,
        )))
    }

    /**
     * Adds a token with a literal to the list of tokens
     */
    fn add_literal_token(&mut self, token_type: TokenType, literal: Literal, src: &str) {
        self.tokens.push(PossibleToken::Ok(Token::new(
            token_type,
            self.get_lexeme(src),
            Some(literal),
            self.line_number,
        )))
    }

    /**
     * Checks if the next grapheme matches the expected string, and if so, advances the iterator
     */
    fn next_matches(
        &mut self,
        grapheme_iter: &mut Peekable<GraphemeIndices>,
        expected: &str,
    ) -> bool {
        if let Some((_, nxt)) = grapheme_iter.peek() {
            if *nxt == expected {
                if let Some((next_idx, _)) = grapheme_iter.next() {
                    self.lexeme_current = next_idx;
                    return true;
                }
            }
        }

        false
    }

    /**
     * Parses a string from the current position
     * Assumes that the current position is a quote
     * If the string is unterminated, an error is added to the list of tokens
     */
    fn parse_string(&mut self, grapheme_iter: &mut Peekable<GraphemeIndices>, src: &str) {
        while let Some((next_idx, g)) = grapheme_iter.next() {
            self.lexeme_current = next_idx;

            if g == "\n" {
                self.line_number += 1;
                continue;
            }

            if g == "\"" {
                // Trim the quotes
                self.lexeme_start += 1;
                self.lexeme_current -= 1;

                self.add_literal_token(String, Literal::String(self.get_lexeme(src)), src);

                // Reset the start and current
                self.lexeme_current += 1;
                self.lexeme_start -= 1;

                return;
            }
        }

        self.tokens.push(PossibleToken::Err(LoxErrorReport::new(
            self.line_number,
            format!(""),
            format!(
                "Unterminated string at line {} pos {}",
                self.line_number, self.lexeme_start
            ),
        )));
    }

    /**
     * Parses a number from the current position
     * Assumes that the current position is a digit
     * Advances the iterator to the end of the number
     * Allows for a single decimal point, but not leading or trailing
     */
    fn parse_number(&mut self, grapheme_iter: &mut Peekable<GraphemeIndices>, src: &str) {
        let mut has_decimal = false;
        while let Some((next_idx, g)) = grapheme_iter.peek() {
            if *g == "." {
                if has_decimal {
                    break;
                }

                has_decimal = true;
            } else if !is_digit(g) {
                break;
            }

            self.lexeme_current = *next_idx;
            grapheme_iter.next();
        }

        let parsed_number = self.get_lexeme(src).parse::<f64>();

        if parsed_number.is_err() {
            self.tokens.push(PossibleToken::Err(LoxErrorReport::new(
                self.line_number,
                format!(""),
                format!(
                    "Invalid number at line {} pos {}",
                    self.line_number, self.lexeme_start
                ),
            )));
            return;
        }

        self.add_literal_token(Number, Literal::Number(parsed_number.unwrap()), src);
    }

    fn parse_identifier(&mut self, grapheme_iter: &mut Peekable<GraphemeIndices>, src: &str) {
        while let Some((next_idx, g)) = grapheme_iter.peek() {
            if !is_alphanumeric(g) {
                break;
            }

            self.lexeme_current = *next_idx;
            grapheme_iter.next();
        }

        let literal = self.get_lexeme(src);
        self.add_literal_token(Identifier, Literal::Identifier(literal), src);
    }
}

/**
 * Checks if the given string is a digit (0-9)
 */
fn is_digit(g: &str) -> bool {
    let char = g.chars().next();

    match char {
        Some(c) => c.is_digit(10),
        None => false,
    }
}

/**
 * Checks if the given string is an alpha character (a-z, A-Z, _)
 */
fn is_alpha(g: &str) -> bool {
    let char = g.chars().next();

    match char {
        Some(c) => c.is_alphabetic() || c == '_',
        None => false,
    }
}

/**
 * Checks if the given string is an alphanumeric character (a-z, A-Z, 0-9, _)
 */
fn is_alphanumeric(g: &str) -> bool {
    let char = g.chars().next();

    match char {
        Some(c) => c.is_alphanumeric() || c == '_',
        None => false,
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    impl PossibleToken {
        pub fn unwrap(self) -> Token {
            match self {
                PossibleToken::Ok(token) => token,
                PossibleToken::Err(err) => panic!("Error token: {}", err.message),
            }
        }
    }

    #[test]
    fn test_is_digit() {
        for i in 0..10 {
            assert!(is_digit(&i.to_string()));
        }

        for c in "abcdefghijklmnopqrstuvwxyz$&~".chars() {
            assert!(!is_digit(&c.to_string()));
        }
    }

    #[rstest]
    #[case::simple_digits(
        "1 < 3 + 4",
        vec![(Number, "1"), (Less, "<"), (Number, "3"), (Plus, "+"), (Number, "4"), (EOF, "")])]
    #[case::digits_with_comments_and_string(
        "1 < 3 + 4 // This is a comment\n\"Hello, world!\" 2 // This is another comment",
        vec![(Number, "1"), (Less, "<"), (Number, "3"), (Plus, "+"), (Number, "4"), (String, "Hello, world!"), (Number, "2"), (EOF, "")])]
    #[case::decimal_number(
        "1.234",
        vec![(Number, "1.234"), (EOF, "")])]
    #[case::complex_decimal_number(
        "1.234.567.123",
        vec![(Number, "1.234"), (Dot, "."), (Number, "567.123"), (EOF, "")])]
    fn test_scan_tokens(#[case] input: &str, #[case] expected: Vec<(TokenType, &str)>) {
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(tokens.len(), expected.len());

        for (i, token) in tokens.iter().enumerate() {
            let token = token.clone().unwrap();
            assert_eq!(token.token_type, expected[i].0);
            assert_eq!(token.lexeme, expected[i].1);
        }
    }

    #[rstest]
    #[case::simple_identifier(
        "a",
        vec![(Identifier, "a"), (EOF, "")])]
    #[case::simple_identifier_with_number(
        "a1",
        vec![(Identifier, "a1"), (EOF, "")])]
    #[case::simple_identifier_with_number_and_underscore(
        "a1_",
        vec![(Identifier, "a1_"), (EOF, "")])]
    #[case::simple_identifier_with_number_and_underscore_and_alpha(
        "a1_b",
        vec![(Identifier, "a1_b"), (EOF, "")])]
    #[case::simple_identifier_with_number_and_underscore_and_alpha_and_comment(
        "a1_b // This is a comment",
        vec![(Identifier, "a1_b"), (EOF, "")])]
    #[case::identifer_starting_with_underscore(
        "_a",
        vec![(Identifier, "_a"), (EOF, "")])]
    fn test_scan_tokens_identifier(#[case] input: &str, #[case] expected: Vec<(TokenType, &str)>) {
        let tokens = Scanner::scan_tokens(input);

        assert_eq!(tokens.len(), expected.len());

        let token = tokens[0].clone().unwrap();

        assert_eq!(token.token_type, expected[0].0);
        assert_eq!(token.lexeme, expected[0].1);

        assert!(token.literal.is_some());
        let literal = token.literal.unwrap();
        assert_eq!(literal, Literal::Identifier(expected[0].1.to_string()));
    }
}
