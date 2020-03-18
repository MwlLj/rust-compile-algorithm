#[derive(Clone, Debug)]
pub enum TokenType {
    None,
    WhiteSpace,
    SplitSymbol,
    SmallParentheses,
    Word
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(token_type: &TokenType, value: String) -> Self {
        let obj = Self {
            token_type: token_type.clone(),
            value: value
        };
        obj
    }
}
