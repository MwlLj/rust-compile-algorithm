use super::token::{Token, TokenType};

#[derive(Debug)]
pub enum Code {
}

type Result<T> = std::result::Result<T, Code>;

pub struct TokenParser {
}

impl TokenParser {
    pub fn parse(&self, sql: &str) -> Result<Vec<Token>> {
        let chars = sql.chars();
        /*
        ** token_type 记录分界点
        */
        let mut token_type = TokenType::None;
        let mut tokens: Vec<Token> = Vec::new();
        for c in chars {
            if c.is_ascii_whitespace() {
                match token_type {
                    TokenType::WhiteSpace => {
                    },
                    _ => {
                        token_type = TokenType::WhiteSpace;
                    }
                }
            } else if c == '(' || c == ')' {
                match token_type {
                    TokenType::SmallParentheses => {
                    },
                    _ => {
                        token_type = TokenType::SmallParentheses;
                        tokens.push(Token::new(&token_type, String::new()));
                    }
                }
                match tokens.last_mut() {
                    Some(last) => {
                        last.value.push(c);
                    },
                    None => {
                        panic!("not happend");
                    }
                }
            } else if c == ',' || c == ';' {
                match token_type {
                    TokenType::SplitSymbol => {
                    },
                    _ => {
                        token_type = TokenType::SplitSymbol;
                        tokens.push(Token::new(&token_type, String::new()));
                    }
                }
                match tokens.last_mut() {
                    Some(last) => {
                        last.value.push(c);
                    },
                    None => {
                        panic!("not happend");
                    }
                }
            } else {
                match token_type {
                    TokenType::Word => {
                    },
                    _ => {
                        token_type = TokenType::Word;
                        tokens.push(Token::new(&token_type, String::new()));
                    }
                }
                match tokens.last_mut() {
                    Some(last) => {
                        last.value.push(c);
                    },
                    None => {
                        panic!("not happend");
                    }
                }
            }
        }
        Ok(tokens)
    }
}

impl TokenParser {
    pub fn new() -> Self {
        let obj = Self{
        };
        obj
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn token_parse_test() {
        let sql = r#"
        CREATE TABLE COMPANY(
           ID INT PRIMARY KEY     NOT NULL,
           NAME           TEXT    NOT NULL,
           AGE            INT     NOT NULL,
           ADDRESS        CHAR(50),
           SALARY         REAL
        );
        "#;
        let parser = TokenParser::new();
        let tokens = parser.parse(sql);
        match tokens {
            Ok(ts) => {
                println!("{:?}", ts);
            },
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
