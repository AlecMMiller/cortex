use rusqlite::Transaction;
use serde_json::Value;

use crate::{
    database::{attribute_schema::RawAttributeSchema, Insert},
    models::{
        attribute_type::{AttributeType, SimpleAttributeType},
        entity::EntityId,
    },
};

impl Insert<EntityId, String> for RawAttributeSchema {
    fn insert(&self, tx: &Transaction, entity: &EntityId, val: &String) -> rusqlite::Result<()> {
        match &self.attr_type {
            AttributeType::Reference(reference) => {
                let target: EntityId = val.try_into().unwrap(); // TODO
                reference.insert_reference(tx, entity, &self.id, &target)
            }
            AttributeType::Simple(simple) => simple.insert_string(tx, entity, &self.id, val),
        }
    }
}

impl Insert<EntityId, Vec<Value>> for RawAttributeSchema {
    fn insert(
        &self,
        tx: &Transaction,
        entity: &EntityId,
        vals: &Vec<Value>,
    ) -> rusqlite::Result<()> {
        match self.attr_type {
            AttributeType::Reference(..) => todo!(),
            AttributeType::Simple(simple) => match simple {
                SimpleAttributeType::Longform => todo!(),
                SimpleAttributeType::Text | SimpleAttributeType::RichText => {
                    simple.insert_string_vec(tx, entity, &self.id, vals)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::{
            attribute_schema::RawAttributeSchema,
            entity::add_entity,
            test::test_util::{setup, ASD, ESD},
            Get, Insert,
        },
        models::attribute_schema::Quantity,
    };
    use rusqlite::Error;

    #[test]
    fn insert_empty_optional() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::default().create(&tx);
        let attr_schema_id = ASD::default()
            .quantity(Quantity::Optional)
            .create(&tx, &schema);

        let attr_schema = RawAttributeSchema::get(&tx, &attr_schema_id).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        let child_id = add_entity(&tx, &schema, data).unwrap();

        attr_schema
            .insert(&tx, &child_id, &"Test".to_string())
            .unwrap();
    }

    #[test]
    fn insert_empty_list() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::default().create(&tx);
        let attr_schema_id = ASD::default().quantity(Quantity::List).create(&tx, &schema);

        let attr_schema = RawAttributeSchema::get(&tx, &attr_schema_id).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
            }}
            "#
        ))
        .unwrap();

        let child_id = add_entity(&tx, &schema, data).unwrap();

        attr_schema
            .insert(&tx, &child_id, &"Test 2".to_string())
            .unwrap();
    }

    #[test]
    fn insert_populated_list() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::default().create(&tx);
        let attr_schema_id = ASD::default().quantity(Quantity::List).create(&tx, &schema);

        let attr_schema = RawAttributeSchema::get(&tx, &attr_schema_id).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr_schema_id}": ["Test"]
            }}
            "#
        ))
        .unwrap();

        let child_id = add_entity(&tx, &schema, data).unwrap();

        attr_schema
            .insert(&tx, &child_id, &"Test 2".to_string())
            .unwrap();
    }

    #[test]
    fn insert_populated_optional_error() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::default().create(&tx);
        let attr_schema_id = ASD::default()
            .quantity(Quantity::Optional)
            .create(&tx, &schema);

        let attr_schema = RawAttributeSchema::get(&tx, &attr_schema_id).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr_schema_id}": "Test"
            }}
            "#
        ))
        .unwrap();

        let child_id = add_entity(&tx, &schema, data).unwrap();

        let result = attr_schema.insert(&tx, &child_id, &"Test 2".to_string());

        assert_eq!(
            result,
            Err(Error::SqliteFailure(
                libsqlite3_sys::Error {
                    code: libsqlite3_sys::ErrorCode::ConstraintViolation,
                    extended_code: 1811
                },
                Some("Attempted to add second entry to non-list field".to_string())
            ))
        );
    }

    #[test]
    fn insert_populated_required_error() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();

        let schema = ESD::default().create(&tx);
        let attr_schema_id = ASD::default().create(&tx, &schema);

        let attr_schema = RawAttributeSchema::get(&tx, &attr_schema_id).unwrap();

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attr_schema_id}": "Test"
            }}
            "#
        ))
        .unwrap();

        let child_id = add_entity(&tx, &schema, data).unwrap();

        let result = attr_schema.insert(&tx, &child_id, &"Test 2".to_string());

        assert_eq!(
            result,
            Err(Error::SqliteFailure(
                libsqlite3_sys::Error {
                    code: libsqlite3_sys::ErrorCode::ConstraintViolation,
                    extended_code: 1811
                },
                Some("Attempted to add second entry to non-list field".to_string())
            ))
        );
    }
}
