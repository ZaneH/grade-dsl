use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct Span {
    column: usize,
}

#[derive(Error, Debug)]
pub enum LexError {
    #[error("Unexpected token at {span:?}")]
    UnexpectedCharacter { span: Span },
}

#[derive(Debug)]
pub enum Token {
    Number { value: f64, span: Span },
    String { value: String, span: Span },
    PercentSign { span: Span },
    OpenSquareBracket { span: Span },
    CloseSquareBracket { span: Span },
    OpenQuoteMark { span: Span },
    CloseQuoteMark { span: Span },
    Comma { span: Span },
    UndeterminedValue { span: Span },
    Course { name: String, span: Span },
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut is_string_open = false;
    let mut string_value = "".to_owned();

    let cursor = Span { column: 0 };
    let tokens = input
        .split_inclusive([' ', '"'])
        .filter_map(|c| match c.trim() {
            "course" => Some(Token::Course {
                name: "Test".to_owned(),
                span: cursor,
            }),
            "\"" => {
                if is_string_open {
                    is_string_open = false;
                    Some(Token::CloseQuoteMark { span: cursor })
                } else {
                    is_string_open = true;
                    Some(Token::OpenQuoteMark { span: cursor })
                }
            }
            _ => {
                if is_string_open {
                    string_value += c;
                }
                Some(Token::String {
                    value: string_value.clone(),
                    span: cursor,
                })
            }
        })
        .collect();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_line() {
        let input = "course \"Test Class\"";
        let res = lex(input).unwrap();
        println!("{:?}", res);
        assert_eq!(res.len(), 4);
    }
}
