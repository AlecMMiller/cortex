use rusqlite::{Error, Result, Transaction};
use serde_json::Value;

use crate::macros::macros::create_id;

use super::{
    attribute_schema::{AttributeSchemaId, RawAttributeSchema},
    entity_schema::EntitySchemaId,
};

create_id!(EntityId);

pub fn new_entity(tx: &Transaction, schema_id: &EntitySchemaId, data: Value) -> Result<EntityId> {
    let data = match data {
        Value::Object(obj) => Ok(obj),
        _ => Err(Error::InvalidQuery),
    }?;

    let id = EntityId::new();

    let schema = RawAttributeSchema::get_for_entity(&tx, schema_id)?;

    tx.execute(
        "INSERT INTO entity (id, schema) VALUES (?1, ?2)",
        (&id, schema_id),
    )?;

    for (key, value) in data {
        let key: AttributeSchemaId = match key.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::InvalidQuery),
        }?;

        let schema_entry = match schema.get(&key) {
            Some(entry) => Ok(entry),
            None => Err(Error::InvalidQuery),
        }?;

        match value {
            Value::String(val) => schema_entry.insert_string(tx, &id, &val),
            _ => Err(Error::InvalidQuery),
        }?;
    }

    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::database::{
        attribute_schema::{AttributeSchema, CreateAttributeSchema, Quantity},
        attribute_type::{CreateAttributeType, SimpleAttributeType},
        entity_schema::{CreateEntitySchema, EntitySchema},
        test::test_util::setup,
    };

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_name = "Foo";
        let attribute_name = "Bar";

        let schema = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: entity_name.to_string(),
            },
        )
        .expect("Unable to create entity");
        let schema_id = schema.id;

        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity: schema_id.clone(),
                name: attribute_name.to_string(),
                quantity: Quantity::Required,
                attr_type: CreateAttributeType::SimpleAttributeType(SimpleAttributeType::Text),
            },
        )
        .expect("Failed to create attribute");
        let attribute_id = new_attribute.id;

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": "Hello world" 
            }}
            "#
        ))
        .unwrap();

        super::new_entity(&tx, &schema_id, data).expect("Failed to create");
    }
}
