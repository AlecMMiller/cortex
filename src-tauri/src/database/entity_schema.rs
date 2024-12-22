use crate::macros::macros::create_id;
use rusqlite::{params, Result, Transaction};
use serde::Serialize;
use specta::Type;

use super::attribute_schema::AttributeSchema;

create_id!(EntitySchemaId);

#[derive(Type, Serialize)]
pub struct EntitySchema {
    pub id: EntitySchemaId,
    pub name: String,
    pub attributes: Vec<AttributeSchema>,
}

pub struct CreateEntitySchema {
    pub name: String,
}

impl EntitySchema {
    pub fn new(conn: &Transaction, data: CreateEntitySchema) -> Result<Self> {
        let new_entity_schema = Self {
            id: EntitySchemaId::new(),
            name: data.name,
            attributes: Vec::new(),
        };

        conn.execute(
            "INSERT INTO entity_schema (id, name) VALUES (?1, ?2)",
            (&new_entity_schema.id, &new_entity_schema.name),
        )?;

        Ok(new_entity_schema)
    }

    pub fn get(tx: &Transaction, id: &EntitySchemaId) -> Result<Self> {
        let attributes = AttributeSchema::get_for_entity(tx, id)?;

        tx.query_row(
            "SELECT id, name FROM entity_schema WHERE id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    attributes,
                })
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{
        attribute_schema::{AttributeType, CreateAttributeSchema},
        test::test_util::setup,
    };

    use super::*;

    const NAME: &str = "Foo";

    fn create(tx: &Transaction) -> EntitySchemaId {
        let new = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: NAME.to_string(),
            },
        )
        .expect("Unable to create entity");

        new.id
    }

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let id = create(&tx);

        let stored = EntitySchema::get(&tx, &id).expect("Could not get stored");

        assert_eq!(stored.id, id);
        assert_eq!(stored.name, NAME);
    }

    #[test]
    fn get_attributes() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_id = create(&tx);

        let attr_type_1 = AttributeType::RichText;
        let name1 = "BAR";

        let attr_type_2 = AttributeType::Text;
        let name2 = "BUZZ";

        AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                name: name1.to_string(),
                entity: entity_id.clone(),
                attr_type: attr_type_1,
            },
        )
        .unwrap();

        AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                name: name2.to_string(),
                entity: entity_id.clone(),
                attr_type: attr_type_2,
            },
        )
        .unwrap();

        let stored = EntitySchema::get(&tx, &entity_id).unwrap();
        let attributes = stored.attributes;

        assert!(attributes.iter().any(|attr| attr.name == name1));
        assert!(attributes.iter().any(|attr| attr.name == name2));
    }
}
