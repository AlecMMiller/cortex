mod add;

use add::add_schema;
use sqlx::SqlitePool;

use crate::{
    database::builder::{
        column::{ColumnBuilder, ColumnIdentifier},
        table::{TableBuilder, TableIdentifier},
    },
    types::{
        column::{DataType, RawDataType},
        schema::Schema,
    },
};

static TABLE_SCHEMA: &str = "table_schema";
static COLUMN_SCHEMA: &str = "column";

pub async fn initialize_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let schema_table_identifier = TableIdentifier::Hardcoded(TABLE_SCHEMA);
    let mut query = TableBuilder::new(schema_table_identifier);
    query.add_column(ColumnBuilder::new(
        ColumnIdentifier::Hardcoded("title"),
        RawDataType::Text,
    ));

    query.execute(pool).await?;

    let mut query = TableBuilder::new(TableIdentifier::Hardcoded(COLUMN_SCHEMA));
    query.add_column(ColumnBuilder::new(
        ColumnIdentifier::Hardcoded("title"),
        RawDataType::Text,
    ));
    query.add_foreign_key(schema_table_identifier);

    query.execute(pool).await?;

    let mut note_schema = Schema::new("Note");
    note_schema.add_column("Title", DataType::Text);
    note_schema.add_column("Content", DataType::RichText);
    add_schema(&note_schema, pool).await?;

    Ok(())
}
