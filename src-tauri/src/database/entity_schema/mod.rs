use crate::{
    models::{
        attribute_schema::AttributeSchema,
        entity_schema::{CreateEntitySchema, EntitySchema, EntitySchemaId},
    },
    utils::get_timestamp,
};
use rusqlite::{params, Result, Transaction};

use super::{Get, GetMany, New};

impl New<CreateEntitySchema> for EntitySchema {
    fn new(conn: &Transaction, data: CreateEntitySchema) -> Result<Self> {
        let new_entity_schema = Self {
            id: EntitySchemaId::new(),
            name: data.name,
            attributes: Vec::new(),
        };

        let created_at = get_timestamp();

        conn.execute(
            "INSERT INTO entity_schema (id, name, created, updated) VALUES (?1, ?2, ?3, ?3)",
            (&new_entity_schema.id, &new_entity_schema.name, created_at),
        )?;

        Ok(new_entity_schema)
    }
}

impl Get<EntitySchemaId> for EntitySchema {
    fn get(tx: &Transaction, id: &EntitySchemaId) -> Result<Self> {
        let attributes = AttributeSchema::get_many(tx, id)?;

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
    use crate::{
        database::{test::test_util::setup, New},
        models::{
            attribute_schema::{CreateAttributeSchema, Quantity},
            attribute_type::{CreateAttributeType, SimpleAttributeType},
        },
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

        let attr_type_1 = CreateAttributeType::SimpleAttributeType(SimpleAttributeType::Text);
        let name1 = "BAR";

        let attr_type_2 = CreateAttributeType::SimpleAttributeType(SimpleAttributeType::Text);
        let name2 = "BUZZ";

        AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                name: name1.to_string(),
                quantity: Quantity::Required,
                entity: entity_id.clone(),
                attr_type: attr_type_1,
            },
        )
        .unwrap();

        AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                name: name2.to_string(),
                quantity: Quantity::Required,
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
