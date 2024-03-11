use std::error::Error;

#[derive(Debug)]
pub struct SyntaxError(pub String);

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax Error: unknown syntax near '{}'", self.0)
    }
}

impl Error for SyntaxError {}
