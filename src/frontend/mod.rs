mod interactive;
mod lex;
mod parse;

use std::{error::Error, fs};

pub use self::interactive::run_interactive;
pub use self::lex::token::Token;

use self::{
    lex::scanner::Scanner,
    parse::{recursive_descent::Parser, tree_walk_interpreter::interpret},
};

pub fn run_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;
    run(&input);
    Ok(())
}

pub fn run(lox_str: &str) {
    let tokens = Scanner::scan_tokens(lox_str);

    let had_error = tokens.iter().any(|t| t.is_err());

    if had_error {
        for error_report in tokens.iter().filter(|t| t.is_err()) {
            let error_report = error_report.as_ref().unwrap_err();
            println!(
                "Error on line {}: {}",
                error_report.line_number, error_report.message
            );
        }
    }

    // unwrap the tokens
    let tokens: Vec<_> = tokens.into_iter().map(|t| t.unwrap()).collect();

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    if let Err(err) = &expr {
        println!("Error on line {}: {}", err.token.line_number, err.message);
        return;
    }

    let result = interpret(&expr.unwrap());
    match result {
        Ok(value) => {
            println!(
                "{}",
                match value {
                    Some(_) => value.unwrap().to_string(),
                    None => "nil".to_string(),
                }
            );
        }
        Err(err) => {
            print!("{}", err.message);
            if let Some(token) = err.token {
                println!(" [line {}]", token.line_number);
            } else {
                println!();
            }
        }
    }
}
