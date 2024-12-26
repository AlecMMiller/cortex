mod add_attribute_schema;
mod get_attribute_schema;
mod utils;

use std::collections::HashMap;

pub use get_attribute_schema::GetSchemaMap;

use crate::models::{
    attribute_schema::{AttributeSchemaId, Quantity},
    attribute_type::{AttributeType, SimpleAttributeType},
    entity::EntityId,
};
use rusqlite::{Result, Transaction};
use serde_json::Value;

use super::Insert;

pub struct RawAttributeSchema {
    pub id: AttributeSchemaId,
    pub attr_type: AttributeType,
    pub quantity: Quantity,
}

pub type SchemaMap = HashMap<AttributeSchemaId, RawAttributeSchema>;

impl Insert<EntityId, String> for RawAttributeSchema {
    fn insert(&self, tx: &Transaction, entity: &EntityId, val: &String) -> Result<()> {
        match &self.attr_type {
            AttributeType::ReferenceAttribute(reference) => {
                let target: EntityId = val.try_into().unwrap(); // TODO
                reference.insert_reference(tx, entity, &self.id, &target)
            }
            AttributeType::SimpleAttributeType(simple) => {
                simple.insert_string(tx, entity, &self.id, val)
            }
        }
    }
}

impl Insert<EntityId, Vec<Value>> for RawAttributeSchema {
    fn insert(&self, tx: &Transaction, entity: &EntityId, vals: &Vec<Value>) -> Result<()> {
        match self.attr_type {
            AttributeType::ReferenceAttribute(..) => todo!(),
            AttributeType::SimpleAttributeType(simple) => match simple {
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
            test::test_util::{
                create_attribute_schema, create_entity_schema, create_reference_schema, setup, ASD,
                ESD, RSD,
            },
            Get,
        },
        models::{
            attribute_schema::AttributeSchema,
            attribute_type::{ReferenceAttribute, SimpleAttributeType},
        },
    };

    use super::*;

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_id = create_entity_schema(&tx, ESD::default());

        let attribute_name = "Bar";

        let new_attribute =
            create_attribute_schema(&tx, entity_id, ASD::default().name(attribute_name));

        let stored = AttributeSchema::get(&tx, &new_attribute).expect("Failed to get stored");

        assert_eq!(stored.id, new_attribute);
        assert_eq!(stored.name, attribute_name);
        assert_eq!(stored.quantity, Quantity::Required);
        assert_eq!(
            stored.attr_type,
            AttributeType::SimpleAttributeType(SimpleAttributeType::Text)
        );
    }

    #[test]
    fn reference() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_name = "Foo";

        let entity_id = create_entity_schema(&tx, ESD::default());
        let attribute_id =
            create_reference_schema(&tx, entity_id.clone(), entity_id.clone(), RSD::default());

        let stored = AttributeSchema::get(&tx, &attribute_id).expect("Failed to get stored");

        assert_eq!(
            stored.attr_type,
            AttributeType::ReferenceAttribute(ReferenceAttribute {
                id: entity_id,
                name: entity_name.to_string()
            })
        );
    }
}
