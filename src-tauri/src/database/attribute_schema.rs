use rusqlite::{params, Result, Transaction};

use crate::macros::macros::create_id;

use super::entity_schema::EntitySchemaId;

create_id!(AttributeSchemaId);

pub struct AttributeSchema {
    id: AttributeSchemaId,
    name: String,
}

pub struct CreateAttributeSchema<'a> {
    entity: &'a EntitySchemaId,
    name: String,
}

impl AttributeSchema {
    fn new(tx: &Transaction, data: CreateAttributeSchema) -> Result<Self> {
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
