/*
** 创建表语法分析
*/
use crate::token::token::Token;
use super::Create;

pub struct FieldDefault {
}

pub struct FieldColumnConstraint {
}

pub struct NameType {
    field_name: Option<Token>,
    field_type: Option<Token>,
    default: Option<FieldDefault>,
    column_constraint: Option<FieldColumnConstraint>
}

pub struct TableConstraint {
}

pub enum RowItem {
    NameType(NameType),
    TableConstraint(TableConstraint)
}

#[derive(Default)]
pub struct TableData {
}

pub struct Table {
    create: Create
}

impl Table {
    pub fn parse(&self) -> TableData {
        let mut table_data = TableData::default();
        table_data
    }
}

impl Table {
    pub fn new(create: Create) -> Self {
        let obj = Self{
            create: create
        };
        obj
    }
}
