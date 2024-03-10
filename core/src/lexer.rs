use crate::{errors::SyntaxError, token::Token};

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = source_code.chars().peekable();
    let mut buffer = String::new();

    while let Some(char) = chars.next() {
        // whitespace is inert, but denotes the end of a multi-character
        // token
        if char.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push(Token::try_from(buffer.as_str())?);
                buffer.clear();
            }

            continue;
        }

        // single-character tokens
        if let Ok(token) = Token::try_from(char) {
            tokens.push(token);
            continue;
        }

        // multi-character tokens
        match char.to_ascii_uppercase() {
            '0'..='9' => {
                buffer.push(char);
            }
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
            | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => {
                buffer.push(char);
            }
            _ => {
                return Err(SyntaxError);
            }
        }

        if chars.peek().is_none() && !buffer.is_empty() {
            tokens.push(Token::try_from(buffer.as_str())?);
            buffer.clear();
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
}
