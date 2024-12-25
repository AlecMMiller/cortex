use rusqlite::{Error, Transaction};
use serde_json::Value;

use super::{
    attribute_schema::{AttributeSchemaId, RawAttributeSchema},
    entity::EntityId,
    entity_schema::EntitySchemaId,
};

pub fn new_entity(
    tx: &Transaction,
    schema_id: &EntitySchemaId,
    data: Value,
) -> rusqlite::Result<EntityId> {
    let data = match data {
        Value::Object(obj) => Ok(obj),
        _ => Err(Error::ModuleError("Data is not an object".to_string())),
    }?;

    let id = EntityId::new();

    let schema = RawAttributeSchema::get_for_entity_schema(&tx, schema_id)?;

    tx.execute(
        "INSERT INTO entity (id, schema) VALUES (?1, ?2)",
        (&id, schema_id),
    )?;

    for (key, value) in data {
        let key: AttributeSchemaId = match key.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::ModuleError("Key not a valid SchemaID".to_string())),
        }?;

        let schema_entry = match schema.get(&key) {
            Some(entry) => Ok(entry),
            None => Err(Error::ModuleError("Key not found in schema".to_string())),
        }?;

        match value {
            Value::String(val) => schema_entry.insert_string(tx, &id, &val),
            Value::Array(vals) => schema_entry.insert_vec(tx, &id, &vals),
            _ => todo!(),
        }?;
    }

    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::database::{
        entity_schema::{CreateEntitySchema, EntitySchema},
        test::test_util::setup,
    };

    use super::*;

    #[test]
    fn simple_entity() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let entity_schema = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: "Entity".to_string(),
            },
        )
        .unwrap()
        .id;
    }
}
