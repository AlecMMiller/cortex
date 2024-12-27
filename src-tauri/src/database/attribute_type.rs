use crate::{
    models::{
        attribute_schema::AttributeSchemaId,
        attribute_type::{
            AttributeType, CreateAttributeType, CreateReferenceAttribute, ReferenceAttribute,
            ReferenceAttributeId, SimpleAttributeType, TextAttributeId,
        },
        entity::EntityId,
        entity_schema::EntitySchemaId,
    },
    utils::get_timestamp,
};
use rusqlite::{
    params,
    types::{FromSqlError, FromSqlResult},
    Error, Result, ToSql, Transaction,
};
use serde_json::Value;

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

impl ReferenceAttribute {
    pub fn insert_reference(
        &self,
        tx: &Transaction,
        entity: &EntityId,
        schema: &AttributeSchemaId,
        value: &EntityId,
    ) -> Result<()> {
        let id = ReferenceAttributeId::new();
        let created_at = get_timestamp();
        tx.execute(
            "INSERT INTO reference_attribute (id, entity, schema, value, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![id, entity, schema, value, created_at],
        )?;
        Ok(())
    }
}

impl SimpleAttributeType {
    pub fn from_sql(value: &str) -> FromSqlResult<Self> {
        match value {
            "Text" => Ok(SimpleAttributeType::Text),
            "RichText" => Ok(SimpleAttributeType::RichText),
            _ => Err(FromSqlError::InvalidType),
        }
    }

    pub fn insert_string(
        &self,
        tx: &Transaction,
        entity: &EntityId,
        schema: &AttributeSchemaId,
        value: &str,
    ) -> Result<()> {
        match self {
            SimpleAttributeType::Text | SimpleAttributeType::RichText => {
                let id = TextAttributeId::new();
                let created_at = get_timestamp();
                tx.execute(
                    "INSERT INTO text_attribute (id, entity, schema, value, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                    params![id, entity, schema, value, created_at],
                )?;
                Ok(())
            }
        }
    }

    pub fn insert_string_vec(
        &self,
        tx: &Transaction,
        entity: &EntityId,
        schema: &AttributeSchemaId,
        vals: &Vec<Value>,
    ) -> Result<()> {
        let created_at = get_timestamp();

        let mut stmt = tx.prepare(
            "INSERT INTO text_attribute (id, entity, schema, value, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        )?;

        for val in vals {
            let val = match val {
                Value::String(val) => Ok(val),
                _ => Err(Error::InvalidQuery),
            }?;
            let id = TextAttributeId::new();
            stmt.execute((id, entity, schema, val, created_at))?;
        }
        Ok(())
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
