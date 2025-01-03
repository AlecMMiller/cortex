use rusqlite::{
    types::{FromSql, FromSqlError},
    ToSql,
};

use crate::models::attribute_schema::Quantity;

impl ToSql for Quantity {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Quantity::Optional => Ok("Optional".into()),
            Quantity::Required => Ok("Required".into()),
            Quantity::List => Ok("List".into()),
        }
    }
}

impl FromSql for Quantity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str()?;
        match value {
            "Optional" => Ok(Quantity::Optional),
            "Required" => Ok(Quantity::Required),
            "List" => Ok(Quantity::List),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
