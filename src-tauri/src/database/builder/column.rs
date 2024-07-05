use crate::types::column::ColumnId;
pub struct ColumnBuilder<'a> {
    column_name: &'a str,
    nullable: bool,
    data_type: DataType,
}

#[allow(dead_code)]
pub enum DataType {
    Text,
    Integer,
    Real,
    Blob,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ColumnIdentifier<'a> {
    Hardcoded(&'static str),
    ColumnId(&'a ColumnId)
}

impl<'a> ColumnBuilder<'a> {
    pub fn new(column_identifier: ColumnIdentifier<'a>, data_type: DataType) -> ColumnBuilder<'a> {
        let column_name = match column_identifier {
            ColumnIdentifier::Hardcoded(name) => name,
            ColumnIdentifier::ColumnId(id) => id.get(),
        };
        ColumnBuilder {
            column_name,
            nullable: false,
            data_type,
        }
    }

    pub fn build(&self) -> String {
        let mut query = format!("{} ", self.column_name);
        match self.data_type {
            DataType::Text => query.push_str("TEXT"),
            DataType::Integer => query.push_str("INTEGER"),
            DataType::Real => query.push_str("REAL"),
            DataType::Blob => query.push_str("BLOB"),
        }
        if !self.nullable {
            query.push_str(" NOT NULL");
        }
        query
    }

    #[allow(dead_code)]
    pub fn nullable(mut self) -> ColumnBuilder<'a> {
        self.nullable = true;
        self
    }
}
