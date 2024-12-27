mod add_attribute_schema;
mod get_attribute_schema;
mod utils;

use std::collections::HashMap;

pub use get_attribute_schema::GetSchemaMap;

use crate::models::{
    attribute_schema::{AttributeSchemaId, Quantity},
    attribute_type::AttributeType,
};

pub struct RawAttributeSchema {
    pub id: AttributeSchemaId,
    pub attr_type: AttributeType,
    pub quantity: Quantity,
}

pub type SchemaMap = HashMap<AttributeSchemaId, RawAttributeSchema>;

#[cfg(test)]
mod tests {
    use crate::{
        database::{
            test::test_util::{setup, ASD, ESD, RSD},
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

        let entity_id = ESD::create_default(&tx);

        let attribute_name = "Bar";

        let new_attribute = ASD::default().name(attribute_name).create(&tx, &entity_id);

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

        let entity_id = ESD::create_default(&tx);
        let attribute_id = RSD::create_default(&tx, &entity_id, &entity_id);

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
