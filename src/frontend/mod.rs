mod error_report;
mod interactive;
mod lex;
mod parse;
mod script_error;

use std::fs;

use crate::frontend::parse::ast_printer::print;

pub use self::error_report::LoxErrorReport;
pub use self::interactive::run_interactive;
pub use self::script_error::LoxScriptError;

use self::{lex::scanner::Scanner, parse::recursive_descent::Parser};

pub fn run_file(file_path: &str) -> Result<(), LoxScriptError> {
    let input = fs::read_to_string(file_path)?;
    run(&input)?;
    Ok(())
}

pub fn run(lox_str: &str) -> Result<(), LoxErrorReport> {
    let mut tokens = Scanner::scan_tokens(lox_str);
    // Remove and print any errors
    tokens.retain(|t| {
        if let Err(err) = t {
            println!("Error on line {}: {}", err.line_number, err.message);
            return false;
        }
        true
    });
    // unwrap the tokens
    let tokens: Vec<_> = tokens.into_iter().map(|t| t.unwrap()).collect();

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Print the AST using the AST printer
    match ast {
        Ok(ast) => println!("{}", print(&ast)),
        Err(err) => println!("Error on line {}: {}", err.token.line_number, err.message),
    }

    Ok(())
}
