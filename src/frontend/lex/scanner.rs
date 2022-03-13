use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::lex::token::TokenType;
use crate::frontend::LoxErrorReport;

use super::token::Token;
use super::token::TokenType::*;

pub enum PossibleToken<'a> {
    Ok(Token<'a>),
    Err(LoxErrorReport),
}

pub fn scan_tokens<'a>(source: &'a str) -> Vec<PossibleToken<'a>> {
    let mut tokens: Vec<PossibleToken> = Vec::new();

    for (line_number, line) in source.lines().enumerate() {
        for (idx, g) in line.graphemes(true).enumerate() {
            let mut add_token = |token_type: TokenType| {
                tokens.push(PossibleToken::Ok(Token::new(token_type, g, line_number)))
            };

            match g {
                "(" => add_token(LeftParen),
                ")" => add_token(RightParen),
                "{" => add_token(LeftBrace),
                "}" => add_token(RightBrace),
                "," => add_token(Comma),
                "." => add_token(Dot),
                "-" => add_token(Minus),
                "+" => add_token(Plus),
                ";" => add_token(Semicolon),
                "*" => add_token(Star),

                _ => tokens.push(PossibleToken::Err(LoxErrorReport::new(
                    line_number,
                    format!(""),
                    format!("Invalid token at line {} pos {}: {}", line_number, idx, g),
                ))),
            }
        }
    }

    tokens
}
