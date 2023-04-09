use std::iter::Peekable;
use std::string::String;

use unicode_segmentation::GraphemeIndices;
use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::lex::token::TokenType;
use crate::frontend::LoxErrorReport;

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

                "!" => add_if_next_matches("=", BangEqual, Bang),
                "=" => add_if_next_matches("=", EqualEqual, Equal),
                "<" => add_if_next_matches("=", LessEqual, Less),
                ">" => add_if_next_matches("=", GreaterEqual, Greater),

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

                "\n" => scanner.line_number += 1,

                "\"" => scanner.parse_string(&mut grapheme_iter, source),

                _ if is_digit(g) => scanner.parse_number(&mut grapheme_iter, source),

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
    fn get_lexeme(&self, line: &str) -> String {
        line[self.lexeme_start..self.lexeme_current + 1].to_string()
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

                self.add_token(TokenType::String, src);

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

        self.add_token(TokenType::Number, src);
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
    #[case(
        "1 < 3 + 4",
        vec![(Number, "1"), (Less, "<"), (Number, "3"), (Plus, "+"), (Number, "4"), (EOF, "")])]
    #[case(
        "1 < 3 + 4 // This is a comment\n\"Hello, world!\" 2 // This is another comment",
        vec![(Number, "1"), (Less, "<"), (Number, "3"), (Plus, "+"), (Number, "4"), (String, "Hello, world!"), (Number, "2"), (EOF, "")])]
    #[case(
        "1.234",
        vec![(Number, "1.234"), (EOF, "")])]
    #[case(
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
}
