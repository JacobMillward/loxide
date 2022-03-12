use core::fmt;
use std::{error, fs, io};

#[derive(Debug)]
pub struct LoxErrorReport {
    line_number: usize,
    location: String,
    message: String,
}

impl fmt::Display for LoxErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line: {}, Loc: {}, Message: {}",
            self.line_number, self.location, self.message,
        )
    }
}

#[derive(Debug)]
pub enum LoxScriptError {
    IoError(io::Error),
    LoxError(LoxErrorReport),
}

impl error::Error for LoxScriptError {}

impl fmt::Display for LoxScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::LoxError(r) => r.fmt(f),
        }
    }
}

impl From<io::Error> for LoxScriptError {
    fn from(err: io::Error) -> Self {
        LoxScriptError::IoError(err)
    }
}

impl From<LoxErrorReport> for LoxScriptError {
    fn from(err: LoxErrorReport) -> Self {
        LoxScriptError::LoxError(err)
    }
}
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
