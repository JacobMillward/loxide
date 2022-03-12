mod error_report;
mod script_error;

use std::fs;

pub use error_report::LoxErrorReport;
pub use script_error::LoxScriptError;

pub fn run_file(file_path: &str) -> Result<(), LoxScriptError> {
    let input = fs::read_to_string(file_path)?;
    run(&input)?;
    Ok(())
}

pub fn run(lox_str: &str) -> Result<(), LoxErrorReport> {
    for token in lox_str.split_whitespace() {
        println!("{}", token);
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
