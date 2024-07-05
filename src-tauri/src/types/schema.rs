use sqlx::{prelude::Type, Encode, Sqlite};
use uuid::Uuid;

use super::column::{Column, DataType};

pub struct Schema<'a> {
    id: SchemaId,
    pub name: &'a str,
    columns: Vec<Column<'a>>
}

#[derive(Encode)]
pub struct SchemaId (String);

impl<'a> Schema<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            id: SchemaId::new(),
            name,
            columns: Vec::new()
        }
    }

    pub fn id(&self) -> &SchemaId {
        &self.id
    }

    pub fn add_column(&mut self, name: &'a str, data_type: DataType) {
        let column = Column::new(name, data_type);
        self.columns.push(column);
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = &Column> {
        self.columns.iter()
    }
}

impl SchemaId {
    pub fn new() -> Self {
        let id = Uuid::new_v4().simple().to_string();
        let id = format!("u_{}", id);
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }   
}

impl Type<Sqlite> for SchemaId {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}
