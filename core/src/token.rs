use crate::errors::SyntaxError;

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator(BinaryOperator),
    Let,
    /// End of file
    Eof,
}

impl TryFrom<char> for Token {
    type Error = SyntaxError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),
            '=' => Ok(Self::Equals),
            '+' => Ok(Self::BinaryOperator(BinaryOperator::Addition)),
            '*' => Ok(Self::BinaryOperator(BinaryOperator::Multiplication)),
            '/' => Ok(Self::BinaryOperator(BinaryOperator::Division)),
            '-' => Ok(Self::BinaryOperator(BinaryOperator::Subtraction)),
            _ => Err(SyntaxError),
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = SyntaxError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().all(|c| c.is_numeric()) {
            return Ok(Token::Number(
                value.parse::<i32>().map_err(|_| SyntaxError)?,
            ));
        }
        Err(SyntaxError)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::BinaryOperator(l0), Self::BinaryOperator(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
