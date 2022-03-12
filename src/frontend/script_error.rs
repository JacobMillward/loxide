use std::{error, fmt, io};

use super::LoxErrorReport;

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
