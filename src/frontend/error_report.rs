use std::fmt;

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
