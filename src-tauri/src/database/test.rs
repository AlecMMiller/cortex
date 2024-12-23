#[cfg(test)]
pub mod test_util {
    use rusqlite::{Connection, Transaction};

    use crate::database::{
        attribute_schema::{AttributeSchema, AttributeSchemaId, CreateAttributeSchema, Quantity},
        attribute_type::{CreateAttributeType, SimpleAttributeType},
        entity_schema::{CreateEntitySchema, EntitySchema, EntitySchemaId},
        migration::migrate,
    };

    pub fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();

        let tx = conn.transaction().unwrap();
        migrate(&tx).unwrap();
        tx.commit().unwrap();

        conn
    }

    pub fn create_entity_schema(tx: &Transaction) -> EntitySchemaId {
        let schema = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: "Foo".to_string(),
            },
        )
        .expect("Unable to create entity");

        schema.id
    }

    pub fn create_attribute_schema(
        tx: &Transaction,
        name: &str,
        entity: EntitySchemaId,
        attr_type: SimpleAttributeType,
        quantity: Quantity,
    ) -> AttributeSchemaId {
        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity,
                name: name.to_string(),
                quantity,
                attr_type: CreateAttributeType::SimpleAttributeType(attr_type),
            },
        )
        .expect("Failed to create attribute");

        new_attribute.id
    }
}
