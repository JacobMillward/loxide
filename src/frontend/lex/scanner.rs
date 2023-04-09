use std::iter::Peekable;
use std::string::String;

use unicode_segmentation::GraphemeIndices;
use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::lex::token::TokenType;
use crate::frontend::LoxErrorReport;

use super::token::Token;
use super::token::TokenType::*;

pub enum PossibleToken {
    Ok(Token),
    Err(LoxErrorReport),
}

pub struct Scanner {
    lexeme_start: usize,
    lexeme_current: usize,
    tokens: Vec<PossibleToken>,
}

impl Scanner {
    pub fn scan_tokens(source: &str) -> Vec<PossibleToken> {
        let mut scanner = Scanner {
            lexeme_start: 0,
            lexeme_current: 0,
            tokens: Vec::new(),
        };

        for (line_number, line) in source.lines().enumerate() {
            scanner.scan_line(line, line_number);
        }

        scanner.tokens
    }

    fn scan_line(&mut self, line: &str, line_number: usize) {
        // Get an iterator over the graphemes in the line
        let mut grapheme_iter = UnicodeSegmentation::grapheme_indices(line, true).peekable();
        self.lexeme_start = 0;
        self.lexeme_current = 0;

        while let Some((grapheme_idx, g)) = grapheme_iter.next() {
            self.lexeme_start = grapheme_idx;
            self.lexeme_current = grapheme_idx;

            match g {
                "(" => self.add_token(LeftParen, line, line_number),
                ")" => self.add_token(RightParen, line, line_number),
                "{" => self.add_token(LeftBrace, line, line_number),
                "}" => self.add_token(RightBrace, line, line_number),
                "," => self.add_token(Comma, line, line_number),
                "." => self.add_token(Dot, line, line_number),
                "-" => self.add_token(Minus, line, line_number),
                "+" => self.add_token(Plus, line, line_number),
                ";" => self.add_token(Semicolon, line, line_number),
                "*" => self.add_token(Star, line, line_number),

                "=" => {
                    let token_type = if self.next_matches(&mut grapheme_iter, "=") {
                        EqualEqual
                    } else {
                        Equal
                    };

                    self.add_token(token_type, line, line_number)
                }

                _ => self.tokens.push(PossibleToken::Err(LoxErrorReport::new(
                    line_number,
                    format!(""),
                    format!(
                        "Invalid token at line {} pos {}: {}",
                        line_number, grapheme_idx, g
                    ),
                ))),
            }
        }
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
    fn add_token(&mut self, token_type: TokenType, line: &str, line_number: usize) {
        self.tokens.push(PossibleToken::Ok(Token::new(
            token_type,
            self.get_lexeme(line),
            None,
            line_number,
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
}
