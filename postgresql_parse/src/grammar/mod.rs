use crate::token::token::{Token, TokenType};

pub enum GrammarItem {
    Create(create::CreateData)
}

#[derive(Default)]
pub struct GrammarData {
    main_type: Option<Token>,
    item: Option<GrammarItem>
}

pub struct Grammar {
    index: usize,
    tokens: Vec<Token>
}

const STMT_CREATE: &str = "create";

impl Grammar {
    pub fn parse(mut self) -> GrammarData {
        let t = match self.lookup_one() {
            Some(t) => t,
            None => {
                panic!("first token is none");
            }
        };
        let mut grammar_data = GrammarData::default();
        match t.token_type {
            TokenType::Word => {
                if t.value == STMT_CREATE {
                    let token = self.take_one().expect("lookup_one success, take_one must be Some");
                    grammar_data.main_type = Some(token);
                    let obj = create::Create::new(self);
                    let create_data = obj.parse();
                    grammar_data.item = Some(GrammarItem::Create(create_data));
                }
            },
            _ => {
                panic!("first token is not word");
            }
        }
        grammar_data
    }

    pub fn lookup_one(&self) -> Option<&Token> {
        self.lookup_next_n(1)
    }

    pub fn lookup_next_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn take_one(&mut self) -> Option<Token> {
        self.take_next_n(1)
    }

    pub fn take_next_n(&mut self, n: usize) -> Option<Token> {
        if n > self.tokens.len() {
            /*
            ** n 大于 tokens 长度 => 无法取出
            */
            return None;
        }
        return Some(self.tokens.remove(0))
    }
}

impl Grammar {
    pub fn new(tokens: Vec<Token>) -> Self {
        let obj = Self{
            index: 0,
            tokens: tokens,
        };
        obj
    }
}

pub mod create;
