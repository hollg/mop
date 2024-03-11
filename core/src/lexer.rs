use std::iter::{self, from_fn};

use crate::{errors::SyntaxError, token::Token};

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = source_code.chars().peekable();

    while let Some(char) = chars.next() {
        // first let's figure out whether this character is the start of a multi-character
        // token
        match char.to_ascii_uppercase() {
            // any char 0..=9 can only appear in a number, so
            // collect all the consecutive number chars from this point
            // into a String and convert it into a Token
            '0'..='9' => {
                let num_string: String = iter::once(char)
                    .chain(from_fn(|| chars.next_if(|c| c.is_ascii_digit())))
                    .collect();
                tokens.push(
                    Token::try_from(num_string.as_str()).map_err(|_| SyntaxError(num_string))?,
                );
            }
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
            | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => {
                let alpha_string: String = iter::once(char)
                    .chain(from_fn(|| chars.next_if(|c| c.is_ascii_alphabetic())))
                    .collect();
                tokens.push(
                    Token::try_from(alpha_string.as_str())
                        .map_err(|_| SyntaxError(alpha_string))?,
                );
            }
            _ => {
                return Err(SyntaxError(char.into()));
            }
        }
        // single-character tokens
        if let Ok(token) = Token::try_from(char) {
            tokens.push(token);
            continue;
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use crate::token::BinaryOperator;

    use super::*;
    #[test]
    fn process_number() {
        let input = "123";

        assert_eq!(
            tokenize(input).expect("should be able to tokenize"),
            vec![Token::Number(123), Token::Eof]
        );
    }

    #[test]
    fn process_simple_sums() {
        let input1 = "1 + 1";
        let input2 = "2 * 2";
        let input3 = "3 - 3";
        let input4 = "4 / 4";

        assert_eq!(
            tokenize(input1).expect("should be able to tokenize"),
            vec![
                Token::Number(1),
                Token::BinaryOperator(BinaryOperator::Addition),
                Token::Number(1),
                Token::Eof
            ]
        );

        assert_eq!(
            tokenize(input2).expect("should be able to tokenize"),
            vec![
                Token::Number(2),
                Token::BinaryOperator(BinaryOperator::Multiplication),
                Token::Number(2),
                Token::Eof
            ]
        );

        assert_eq!(
            tokenize(input3).expect("should be able to tokenize"),
            vec![
                Token::Number(3),
                Token::BinaryOperator(BinaryOperator::Subtraction),
                Token::Number(3),
                Token::Eof
            ]
        );

        assert_eq!(
            tokenize(input4).expect("should be able to tokenize"),
            vec![
                Token::Number(4),
                Token::BinaryOperator(BinaryOperator::Division),
                Token::Number(4),
                Token::Eof
            ]
        );
    }

    #[test]
    fn process_simple_number_assignment() {
        let input = "let x = 10";

        assert_eq!(
            tokenize(input).expect("should be able to tokenize"),
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Number(10),
                Token::Eof
            ]
        );
    }
}
