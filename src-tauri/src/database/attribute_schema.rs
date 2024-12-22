use rusqlite::{params, Result, Transaction};
use serde::Serialize;
use specta::Type;

use crate::macros::macros::create_id;

use super::entity_schema::EntitySchemaId;

create_id!(AttributeSchemaId);

#[derive(Type, Serialize)]
pub struct AttributeSchema {
    pub id: AttributeSchemaId,
    pub name: String,
}

pub struct CreateAttributeSchema<'a> {
    pub entity: &'a EntitySchemaId,
    pub name: String,
}

impl AttributeSchema {
    pub fn new(tx: &Transaction, data: CreateAttributeSchema) -> Result<Self> {
        let new_attribute = Self {
            id: AttributeSchemaId::new(),
            name: data.name,
        };

        tx.execute(
            "INSERT INTO attribute_schema (id, entity, name) VALUES (?1, ?2, ?3)",
            (&new_attribute.id, data.entity, &new_attribute.name),
        )?;

        Ok(new_attribute)
    }

    fn get(tx: &Transaction, id: &AttributeSchemaId) -> Result<Self> {
        tx.query_row(
            "SELECT id, name FROM attribute_schema WHERE id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )
    }

    pub fn get_for_entity(tx: &Transaction, id: &EntitySchemaId) -> Result<Vec<Self>> {
        let mut statement = tx.prepare("SELECT id, name FROM attribute_schema WHERE entity=?1")?;
        let mut rows = statement.query(params![id])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(Self {
                id: row.get(0)?,
                name: row.get(1)?,
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
                entity: &entity_id,
                name: attribute_name.to_string(),
            },
        )
        .unwrap();
        let attribute_id = new_attribute.id;

        let stored = AttributeSchema::get(&tx, &attribute_id).unwrap();

        assert_eq!(stored.id, attribute_id);
        assert_eq!(stored.name, attribute_name);
    }
}
