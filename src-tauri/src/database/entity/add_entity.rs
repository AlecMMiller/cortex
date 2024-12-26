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
    use crate::database::test::test_util::{
        create_attribute_schema, create_entity_schema, setup, ASD, ESD,
    };

    use super::*;

    // Create an entity with no data
    #[test]
    fn simple_entity() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = create_entity_schema(&tx, ESD::default());

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

        let schema = create_entity_schema(&tx, ESD::default());
        create_attribute_schema(
            &tx,
            schema.clone(),
            ASD::default().quantity(Quantity::Optional),
        );

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

        let schema = create_entity_schema(&tx, ESD::default());
        create_attribute_schema(&tx, schema.clone(), ASD::default());

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

        let schema = create_entity_schema(&tx, ESD::default());
        create_attribute_schema(&tx, schema.clone(), ASD::default());

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

        let schema = create_entity_schema(&tx, ESD::default());
        let attr = create_attribute_schema(&tx, schema.clone(), ASD::default());

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

        let schema = create_entity_schema(&tx, ESD::default());
        let other_schema = create_entity_schema(
            &tx,
            ESD {
                name: "Other".to_string(),
            },
        );
        let attr = create_attribute_schema(&tx, other_schema.clone(), ASD::default());

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
        let schema_id = create_entity_schema(&tx, ESD::default());
        let attribute_id = create_attribute_schema(
            &tx,
            schema_id.clone(),
            ASD::default().quantity(Quantity::List),
        );

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
        let schema_id = create_entity_schema(&tx, ESD::default());
        let attribute_id = create_attribute_schema(
            &tx,
            schema_id.clone(),
            ASD::default().quantity(Quantity::List),
        );

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
        let schema_id = create_entity_schema(&tx, ESD::default());
        let attribute_id = create_attribute_schema(&tx, schema_id.clone(), ASD::default());

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
        let schema_id = create_entity_schema(&tx, ESD::default());
        let attribute_id = create_attribute_schema(
            &tx,
            schema_id.clone(),
            ASD::default().quantity(Quantity::Optional),
        );

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
