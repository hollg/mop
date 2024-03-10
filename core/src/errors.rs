use std::error::Error;

#[derive(Debug)]
pub struct SyntaxError;
impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax Error")
    }
}

impl Error for SyntaxError {}
