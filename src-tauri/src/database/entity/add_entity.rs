use rusqlite::{Error, Transaction};
use serde_json::Value;

use crate::{
    database::{
        attribute_schema::{GetSchemaMap, RawAttributeSchema},
        Insert,
    },
    models::{
        attribute_schema::{AttributeSchemaId, Quantity},
        entity::EntityId,
        entity_schema::EntitySchemaId,
    },
};

pub fn add_entity(
    tx: &Transaction,
    schema_id: &EntitySchemaId,
    data: Value,
) -> rusqlite::Result<EntityId> {
    let data = match data {
        Value::Object(obj) => Ok(obj),
        _ => Err(Error::ModuleError(
            "Provided data is not an object".to_string(),
        )),
    }?;

    let id = EntityId::new();

    let schema = RawAttributeSchema::get_map(&tx, schema_id)?;

    for (schema_id, value) in &schema {
        match value.quantity {
            Quantity::Required => {
                let provided = data.get(&schema_id.to_string());
                match provided {
                    None => Err(Error::ModuleError(
                        "Did not provide field required by schema".to_string(),
                    )),
                    _ => Ok(()),
                }?;
            }
            _ => continue,
        }
    }

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

        let quantity = &schema_entry.quantity;

        match quantity {
            Quantity::Required | Quantity::Optional => match value {
                Value::Array(..) => Err(Error::ModuleError(
                    "Provided a list to a non-list field".to_string(),
                )),
                Value::Number(..) | Value::String(..) | Value::Object(..) => Ok(()),
                _ => todo!(),
            },
            Quantity::List => Ok(()),
        }?;

        match value {
            Value::String(val) => schema_entry.insert(tx, &id, &val),
            Value::Array(vals) => schema_entry.insert(tx, &id, &vals),
            _ => todo!(),
        }?;
    }

    Ok(id)
}

#[cfg(test)]
mod tests {
    use futures::task::waker;

    use crate::database::test::test_util::{setup, ASD, ESD};

    use super::*;

    // Create an entity with no data
    #[test]
    fn simple_entity() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema, data).unwrap();
    }

    // If a field is optional, it should be able to be created without it
    #[test]
    fn optional_attr() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);
        ASD::default()
            .quantity(Quantity::Optional)
            .create(&tx, &schema);

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema, data).unwrap();
    }

    // If a required field is missing, it should throw an error
    #[test]
    fn missing_attr() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);
        ASD::create_default(&tx, &schema);

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        let result = add_entity(&tx, &schema, data);
        assert_eq!(
            result,
            Err(Error::ModuleError(
                "Did not provide field required by schema".to_string()
            ))
        );
    }

    // If the provided data is not an object, it should throw an error
    #[test]
    fn non_object() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);
        ASD::create_default(&tx, &schema);

        let data = serde_json::from_str(&format!(
            r#"
            ["Hello world"] 
            "#
        ))
        .unwrap();

        let result = add_entity(&tx, &schema, data);
        assert_eq!(
            result,
            Err(Error::ModuleError(
                "Provided data is not an object".to_string()
            ))
        );
    }

    // Create an entity with data
    #[test]
    fn data_provided() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);
        let attr = ASD::create_default(&tx, &schema);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "Hello world"
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema, data).unwrap();
    }

    // If an entity is created with an attribute not in its schema, it should throw an error
    #[test]
    fn wrong_data_provided() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::create_default(&tx);
        let other_schema = ESD::default().name("Other").create(&tx);

        let attr = ASD::create_default(&tx, &other_schema);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr}": "Hello world"
            }}
            "#
        ))
        .unwrap();

        let result = add_entity(&tx, &schema, data);
        assert_eq!(
            result,
            Err(Error::ModuleError("Key not found in schema".to_string()))
        );
    }

    // It should accept lists
    #[test]
    fn list() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema_id = ESD::create_default(&tx);
        let attribute_id = ASD::default()
            .quantity(Quantity::List)
            .create(&tx, &schema_id);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": ["Hello world", "Hello moon"] 
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema_id, data).unwrap();
    }

    // It should accept empty lists
    #[test]
    fn empty_list() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema_id = ESD::create_default(&tx);
        let attribute_id = ASD::default()
            .quantity(Quantity::List)
            .create(&tx, &schema_id);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": [] 
            }}
            "#
        ))
        .unwrap();

        add_entity(&tx, &schema_id, data).unwrap();
    }

    // It should throw an error if a list is provided to a field marked required
    #[test]
    fn too_many_args_required() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema_id = ESD::create_default(&tx);
        let attribute_id = ASD::create_default(&tx, &schema_id);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": ["Hello world", "Hello moon"] 
            }}
            "#
        ))
        .unwrap();

        let result = add_entity(&tx, &schema_id, data);
        assert_eq!(
            result,
            Err(Error::ModuleError(
                "Provided a list to a non-list field".to_string()
            ))
        );
    }

    // It should throw an error if a list is provided to a field marked optional
    #[test]
    fn too_many_args_optional() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema_id = ESD::create_default(&tx);
        let attribute_id = ASD::default()
            .quantity(Quantity::Optional)
            .create(&tx, &schema_id);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": ["Hello world", "Hello moon"] 
            }}
            "#
        ))
        .unwrap();

        let result = add_entity(&tx, &schema_id, data);
        assert_eq!(
            result,
            Err(Error::ModuleError(
                "Provided a list to a non-list field".to_string()
            ))
        );
    }
}
