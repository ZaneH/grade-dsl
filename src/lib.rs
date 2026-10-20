use regex::regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("Unexpected token")]
    UnexpectedCharacter,
}

#[derive(Debug)]
pub enum TokenType {
    Number { value: f64 },
    String { value: String },
    PercentSign,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenQuoteMark,
    CloseQuoteMark,
    Comma,
    UndeterminedValue,
    Course { name: String },
}

struct Scanner {
    start: u64,
    line: u64,
    current: u64,

    source: String,
    tokens: Vec<Token>,
}

#[derive(Debug)]
struct Token {
    type_: TokenType,
    text: String,
    literal: u64, // TODO: Change type of literal
    line: u64,
}

impl Scanner {
    fn new(source: &str) -> Self {
        return Self {
            start: 0,
            line: 0,
            current: 0,
            source: source.into(),
            tokens: vec![],
        };
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        if let c = c.unwrap() {
            let r = regex!(r"[a-zA-Z_][a-zA-Z_0-9]*");
            if r.is_match(&c.to_string()) {
                println!("{:?}", c);
            }
        }
    }

    // TODO: Change type of literal
    fn add_token(&mut self, type_: TokenType, literal: u64) {
        self.tokens.push(Token {
            type_,
            text: "Test".into(),
            literal: 0,
            line: self.line,
        })
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        return self.source.chars().nth(self.current as usize);
    }

    fn is_at_end(&self) -> bool {
        return self.current as usize >= self.source.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_line() {
        let input = "course \"Test Class\"";
        let mut s = Scanner::new(input);
        s.scan_tokens();
        println!("{:?}", s.tokens);
    }
}
