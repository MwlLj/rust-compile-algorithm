use crate::token::token::Token;
use super::Grammar;

const STMT_TABLE: &str = "table";

pub enum CreateItem {
    Table(table::TableData)
}

#[derive(Default)]
pub struct CreateData {
    attribute: Option<Vec<Token>>,
    sub_type: Option<Vec<Token>>,
    item: Option<CreateItem>
}

pub struct Create {
    grammar: Grammar
}

impl Create {
    pub fn parse(mut self) -> CreateData {
        let mut create_data = CreateData::default();
        while let Some(t) = self.grammar.lookup_one() {
            /*
            ** 可优化位置
            **  不对 t.value 进行 clone, 使用 t.value 的引用进行比对, 但是每一个分支都需要写
            **  let token = self.grammar.take_one().expect("lookup_one success, take_one must be Some");
            **  (因为 t的不可变引用会在可变引用后使用 => 悬垂指针)
            */
            let value = t.value.clone();
            let token = self.grammar.take_one().expect("lookup_one success, take_one must be Some");
            if value == STMT_TABLE {
                create_data.sub_type = Some(vec![token]);
                let table_obj = table::Table::new(self);
                let table_data = table_obj.parse();
                create_data.item = Some(CreateItem::Table(table_data));
                break;
            } else {
                match &mut create_data.attribute {
                    Some(ts) => {
                        ts.push(token);
                    },
                    None => {
                        create_data.attribute = Some(vec![token]);
                    }
                }
            }
        };
        create_data
    }

    pub fn lookup_one(&self) -> Option<&Token> {
        self.grammar.lookup_one()
    }

    pub fn lookup_next_n(&self, n: usize) -> Option<&Token> {
        self.grammar.lookup_next_n(n)
    }

    pub fn take_one(&mut self) -> Option<Token> {
        self.grammar.take_one()
    }

    pub fn take_next_n(&mut self, n: usize) -> Option<Token> {
        self.grammar.take_next_n(n)
    }
}

impl Create {
    pub fn new(grammar: Grammar) -> Self {
        let obj = Self {
            grammar: grammar
        };
        obj
    }
}

pub mod table;
