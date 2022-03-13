use std::fmt;

#[derive(Debug)]
pub struct LoxErrorReport {
    pub line_number: usize,
    pub location: String,
    pub message: String,
}

impl LoxErrorReport {
    pub fn new(line_number: usize, location: String, message: String) -> LoxErrorReport {
        LoxErrorReport {
            line_number,
            location,
            message,
        }
    }
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
