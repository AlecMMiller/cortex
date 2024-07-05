use sqlx::{query::Query, sqlite::SqliteQueryResult, Error, Sqlite};

use crate::types::schema::SchemaId;

use super::{column::ColumnBuilder, foreign_key::ForeignKeyBuilder};

pub struct TableBuilder<'a> {
    table_name: &'a str,
    columns: Vec<ColumnBuilder<'a>>,
    foreign_keys: Vec<ForeignKeyBuilder<'a>>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TableIdentifier<'a> {
    Hardcoded(&'static str),
    SchemaId(&'a SchemaId)
}

impl<'a> TableIdentifier<'a> {
    pub fn get(&self) -> &'a str {
        match self {
            TableIdentifier::Hardcoded(name) => name,
            TableIdentifier::SchemaId(id) => id.get(),
        }
    }
}

impl<'a> TableBuilder<'a> {
    pub fn new(identifier: TableIdentifier<'a>) -> TableBuilder<'a> {
        let table_name = match identifier {
            TableIdentifier::Hardcoded(name) => name,
            TableIdentifier::SchemaId(id) => id.get(),
        };
        TableBuilder {
            table_name,
            columns: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }

    fn build(&self) -> String {
        let mut query = format!("CREATE TABLE IF NOT EXISTS {} (\n", self.table_name);

        query.push_str("id TEXT PRIMARY KEY NOT NULL,\n");

        for column in self.columns.iter() {
            query.push_str(&column.build());
            query.push_str(",\n");
        }

        for foreign_key in self.foreign_keys.iter() {
            query.push_str(&foreign_key.build());
            query.push_str(",\n");
        }

        query.pop();
        query.pop();
        query.push_str("\n);");

        query
    }

    pub async fn execute(&self, pool: &sqlx::SqlitePool) -> Result<SqliteQueryResult, Error> {
        let query = self.build();
        let query = sqlx::query(query.as_str());
        query.execute(pool).await
    }

    pub fn add_column(&mut self, column: ColumnBuilder<'a>) {
        self.columns.push(column);
    }

    pub fn add_foreign_key(&mut self, table_identifier: TableIdentifier<'a>) {
        let builder = ForeignKeyBuilder::new(table_identifier);
        self.foreign_keys.push(builder);
    }
}
