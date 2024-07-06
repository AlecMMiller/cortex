use sqlx::{query::Query, sqlite::SqliteQueryResult, QueryBuilder, Sqlite, SqlitePool};

use crate::{
    database::builder::{
        column::{ColumnBuilder, ColumnIdentifier},
        table::{TableBuilder, TableIdentifier},
    },
    types::schema::Schema,
};

use super::{COLUMN_SCHEMA, TABLE_SCHEMA};

pub async fn insert_table<'a>(
    schema: &'a Schema<'a>,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let table_insert_query = format!("INSERT INTO {} (id, title) VALUES (?1, ?2);", TABLE_SCHEMA);
    let table_insert_query: Query<Sqlite, _> = sqlx::query(table_insert_query.as_str())
        .bind(schema.id())
        .bind(schema.name);

    table_insert_query.execute(pool).await
}

pub async fn create_table<'a>(
    schema: &'a Schema<'a>,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut table_create_query = TableBuilder::new(TableIdentifier::SchemaId(schema.id()));
    for column in schema.iter_columns() {
        table_create_query.add_column(ColumnBuilder::new(
            ColumnIdentifier::ColumnId(column.id()),
            column.get_raw_data_type(),
        ));
    }

    table_create_query.execute(pool).await
}

pub async fn add_columns<'a>(
    schema: &'a Schema<'a>,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let column_insert_query = format!(
        "INSERT INTO {} ({}, id, title, data_type) ",
        COLUMN_SCHEMA, TABLE_SCHEMA
    );
    let mut column_insert_query = QueryBuilder::new(column_insert_query);

    column_insert_query.push_values(schema.iter_columns(), |mut b, column| {
        b.push_bind(schema.id())
            .push_bind(column.id())
            .push_bind(column.name)
            .push_bind(column.get_data_type_str());
    });

    let column_insert_query = column_insert_query.build();

    column_insert_query.execute(pool).await
}

pub async fn add_schema<'a>(schema: &'a Schema<'a>, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await?;

    let schema_insert_query = insert_table(schema, pool);
    let table_create_query = create_table(schema, pool);
    let column_insert_query = add_columns(schema, pool);

    tokio::try_join!(schema_insert_query, table_create_query, column_insert_query)?;
    transaction.commit().await
}
