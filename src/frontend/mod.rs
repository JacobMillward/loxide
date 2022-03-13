mod error_report;
mod interactive;
mod lex;
mod script_error;

use std::fs;

pub use error_report::LoxErrorReport;
pub use interactive::run_interactive;
pub use script_error::LoxScriptError;

use self::lex::scanner::{scan_tokens, PossibleToken};

pub fn run_file(file_path: &str) -> Result<(), LoxScriptError> {
    let input = fs::read_to_string(file_path)?;
    run(&input)?;
    Ok(())
}

pub fn run(lox_str: &str) -> Result<(), LoxErrorReport> {
    for p_token in scan_tokens(lox_str) {
        match p_token {
            PossibleToken::Ok(token) => {
                println!("{}", token.lexeme);
            }
            PossibleToken::Err(err) => {
                println!("{}", err.message);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
