use rusqlite::{
    params,
    types::{FromSqlError, FromSqlResult, ValueRef},
    Result, ToSql, Transaction,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::entity_schema::EntitySchemaId;

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub struct CreateReferenceAttribute {
    pub id: EntitySchemaId,
}

impl CreateReferenceAttribute {
    fn get_full(&self, tx: &Transaction) -> Result<ReferenceAttribute> {
        tx.query_row(
            "SELECT id, name FROM entity_schema WHERE id=?1",
            params![self.id],
            |row| {
                Ok(ReferenceAttribute {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )
    }
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub struct ReferenceAttribute {
    pub id: EntitySchemaId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
pub enum SimpleAttributeType {
    Text,
    RichText,
}

impl SimpleAttributeType {
    fn from_sql(value: &str) -> FromSqlResult<Self> {
        match value {
            "Text" => Ok(SimpleAttributeType::Text),
            "RichText" => Ok(SimpleAttributeType::RichText),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for SimpleAttributeType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            SimpleAttributeType::Text => Ok("Text".into()),
            SimpleAttributeType::RichText => Ok("RichText".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum CreateAttributeType {
    SimpleAttributeType(SimpleAttributeType),
    CreateReferenceAttribute(CreateReferenceAttribute),
}

impl CreateAttributeType {
    pub fn get_ref(&self) -> Option<&EntitySchemaId> {
        match self {
            CreateAttributeType::SimpleAttributeType(_type) => None,
            CreateAttributeType::CreateReferenceAttribute(reference) => Some(&reference.id),
        }
    }

    pub fn get_full(&self, tx: &Transaction) -> Result<AttributeType> {
        match self {
            CreateAttributeType::SimpleAttributeType(simple) => {
                Ok(AttributeType::SimpleAttributeType(simple.clone()))
            }
            CreateAttributeType::CreateReferenceAttribute(reference) => {
                Ok(AttributeType::ReferenceAttribute(reference.get_full(tx)?))
            }
        }
    }
}

impl ToSql for CreateAttributeType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            CreateAttributeType::SimpleAttributeType(simple) => simple.to_sql(),
            CreateAttributeType::CreateReferenceAttribute(_val) => Ok("Reference".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum AttributeType {
    SimpleAttributeType(SimpleAttributeType),
    ReferenceAttribute(ReferenceAttribute),
}

impl AttributeType {
    pub fn columns_result(
        type_column: ValueRef<'_>,
        id_column: ValueRef<'_>,
        name_column: ValueRef<'_>,
    ) -> FromSqlResult<Self> {
        let value = type_column.as_str()?;
        match value {
            "Reference" => {
                let name = name_column.as_str()?;
                let id = EntitySchemaId::column_result_manual(id_column)?;

                let reference = ReferenceAttribute {
                    id,
                    name: name.into(),
                };

                Ok(AttributeType::ReferenceAttribute(reference))
            }
            simple => Ok(AttributeType::SimpleAttributeType(
                SimpleAttributeType::from_sql(simple)?,
            )),
        }
    }
}
