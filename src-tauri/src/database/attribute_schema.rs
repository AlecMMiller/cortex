use crate::macros::macros::create_id;
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
    Result, ToSql, Transaction,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::entity_schema::EntitySchemaId;

create_id!(AttributeSchemaId);

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum AttributeType {
    Text,
    RichText,
}

impl FromSql for AttributeType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let value = value.as_str()?;
        match value {
            "Text" => Ok(AttributeType::Text),
            "RichText" => Ok(AttributeType::RichText),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for AttributeType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            AttributeType::Text => Ok("Text".into()),
            AttributeType::RichText => Ok("RichText".into()),
        }
    }
}

#[derive(Type, Serialize)]
pub struct AttributeSchema {
    pub id: AttributeSchemaId,
    pub name: String,
    pub attr_type: AttributeType,
}

#[derive(Type, Deserialize)]
pub struct CreateAttributeSchema {
    pub entity: EntitySchemaId,
    pub name: String,
    pub attr_type: AttributeType,
}

impl AttributeSchema {
    pub fn new(tx: &Transaction, data: CreateAttributeSchema) -> Result<Self> {
        let new_attribute = Self {
            id: AttributeSchemaId::new(),
            name: data.name,
            attr_type: data.attr_type,
        };

        tx.execute(
            "INSERT INTO attribute_schema (id, entity, name, type) VALUES (?1, ?2, ?3, ?4)",
            (
                &new_attribute.id,
                data.entity,
                &new_attribute.name,
                &new_attribute.attr_type,
            ),
        )?;

        Ok(new_attribute)
    }

    fn get(tx: &Transaction, id: &AttributeSchemaId) -> Result<Self> {
        tx.query_row(
            "SELECT id, name, type FROM attribute_schema WHERE id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    attr_type: row.get(2)?,
                })
            },
        )
    }

    pub fn get_for_entity(tx: &Transaction, id: &EntitySchemaId) -> Result<Vec<Self>> {
        let mut statement =
            tx.prepare("SELECT id, name, type FROM attribute_schema WHERE entity=?1")?;
        let mut rows = statement.query(params![id])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                attr_type: row.get(2)?,
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{
        entity_schema::{CreateEntitySchema, EntitySchema},
        test::test_util::setup,
    };

    use super::*;

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_name = "Foo";
        let attribute_name = "Bar";

        let entity = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: entity_name.to_string(),
            },
        )
        .expect("Unable to create entity");
        let entity_id = entity.id;

        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity: entity_id,
                name: attribute_name.to_string(),
                attr_type: AttributeType::Text,
            },
        )
        .unwrap();
        let attribute_id = new_attribute.id;

        let stored = AttributeSchema::get(&tx, &attribute_id).unwrap();

        assert_eq!(stored.id, attribute_id);
        assert_eq!(stored.name, attribute_name);
        assert_eq!(stored.attr_type, AttributeType::Text);
    }
}
