use thiserror::Error;

#[derive(Debug)]
pub enum Span {
    Row(usize),
    Column(usize),
}

#[derive(Error, Debug)]
pub enum LexError {
    #[error("Unexpected token `{found:?}` at {span:?}")]
    UnexpectedToken { found: String, span: Span },

    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug)]
pub enum Token {
    Whitespace,
    Number,
    PercentSign,
    OpenSquareBracket,
    Comma,
    CloseSquareBracket,
    UndeterminedValue,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    return Ok(vec![Token::Number]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "midterm 25% [88.5, ?]";
        println!("{:?}", lex(input).unwrap());
    }
}
