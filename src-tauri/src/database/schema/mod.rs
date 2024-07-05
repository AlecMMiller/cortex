use sqlx::{query::Query, Sqlite, SqlitePool};

use crate::database::builder::{column::{ColumnBuilder, ColumnIdentifier, DataType}, table::{TableBuilder, TableIdentifier}};

pub async fn initialize_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let schema_table_identifier = TableIdentifier::Hardcoded("table");
    let mut query = TableBuilder::new(schema_table_identifier);
    query.add_column(ColumnBuilder::new(ColumnIdentifier::Hardcoded("title"), DataType::Text));

    let query = query.build();
    let query: Query<Sqlite, _> = sqlx::query(query.as_str());
    query.execute(pool).await?;

    let mut query = TableBuilder::new(TableIdentifier::Hardcoded("column"));
    query.add_column(ColumnBuilder::new(ColumnIdentifier::Hardcoded("title"), DataType::Text));
    query.add_foreign_key(schema_table_identifier);

    let query = query.build();
    let query: Query<Sqlite, _> = sqlx::query(query.as_str());
    query.execute(pool).await?;

    Ok(())
}
