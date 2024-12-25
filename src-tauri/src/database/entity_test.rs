use serde_json::Value;

use crate::database::{
    attribute_schema::{AttributeSchema, CreateAttributeSchema, Quantity},
    attribute_type::{CreateAttributeType, CreateReferenceAttribute, SimpleAttributeType},
    entity::{get, new_entity, EntityField, EntityRequest},
    entity_schema::{CreateEntitySchema, EntitySchema},
    test::test_util::{create_attribute_schema, create_entity_schema, setup},
};

use super::entity::EntityAttribute;

#[test]
fn text() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();
    let schema_id = create_entity_schema(&tx);
    let attribute_id = create_attribute_schema(
        &tx,
        "Bar",
        schema_id.clone(),
        SimpleAttributeType::Text,
        Quantity::Required,
    );

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_id}": "Hello world" 
            }}
            "#
    ))
    .unwrap();

    let entity_id = new_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![EntityField::Attribute(attribute_id.clone())],
    };

    let result = get(&tx, &entity_id, request).unwrap();

    let val = result.get(&attribute_id.to_string()).unwrap();

    assert_eq!(val, &Value::String("Hello world".to_string()));
}

#[test]
fn list() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();
    let schema_id = create_entity_schema(&tx);
    let attribute_id = create_attribute_schema(
        &tx,
        "Bar",
        schema_id.clone(),
        SimpleAttributeType::Text,
        Quantity::List,
    );

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_id}": ["Hello world", "Hello moon"] 
            }}
            "#
    ))
    .unwrap();

    let entity_id = new_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![EntityField::Attribute(attribute_id.clone())],
    };

    let result = get(&tx, &entity_id, request).unwrap();
    let val = result.get(&attribute_id.to_string()).unwrap();

    let val_1 = Value::String("Hello moon".to_string());
    let val_2 = Value::String("Hello world".to_string());

    let vec = vec![val_1, val_2];
    let expected = Value::Array(vec);

    assert_eq!(val, &expected);
}

#[test]
fn multifield() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();
    let schema_id = create_entity_schema(&tx);
    let attribute_1_id = create_attribute_schema(
        &tx,
        "1",
        schema_id.clone(),
        SimpleAttributeType::Text,
        Quantity::Required,
    );

    let attribute_2_id = create_attribute_schema(
        &tx,
        "2",
        schema_id.clone(),
        SimpleAttributeType::Text,
        Quantity::Required,
    );

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_1_id}": "Message 1",
              "{attribute_2_id}": "Message 2"
            }}
            "#
    ))
    .unwrap();

    let entity_id = new_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![
            EntityField::Attribute(attribute_1_id.clone()),
            EntityField::Attribute(attribute_2_id.clone()),
        ],
    };

    let result = get(&tx, &entity_id, request).unwrap();

    let val_1 = result.get(&attribute_1_id.to_string()).unwrap();
    let val_2 = result.get(&attribute_2_id.to_string()).unwrap();
    assert_eq!(val_1, &Value::String("Message 1".to_string()));
    assert_eq!(val_2, &Value::String("Message 2".to_string()));
}

#[test]
fn reference() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();

    let parent_schema = EntitySchema::new(
        &tx,
        CreateEntitySchema {
            name: "Parent".to_string(),
        },
    )
    .unwrap()
    .id;

    let child_schema = EntitySchema::new(
        &tx,
        CreateEntitySchema {
            name: "Child".to_string(),
        },
    )
    .unwrap()
    .id;

    let reference_attr = AttributeSchema::new(
        &tx,
        CreateAttributeSchema {
            entity: parent_schema.clone(),
            name: "Child".to_string(),
            quantity: Quantity::Required,
            attr_type: CreateAttributeType::CreateReferenceAttribute(CreateReferenceAttribute {
                id: child_schema.clone(),
            }),
        },
    )
    .unwrap()
    .id;

    let child_attr = create_attribute_schema(
        &tx,
        "1",
        child_schema.clone(),
        SimpleAttributeType::Text,
        Quantity::Required,
    );

    let child_data = serde_json::from_str(&format!(
        r#"
            {{
              "{child_attr}": "Message 1"
            }}
            "#
    ))
    .unwrap();

    let child_id = new_entity(&tx, &child_schema, child_data).unwrap();

    let parent_data = serde_json::from_str(&format!(
        r#"
            {{
              "{reference_attr}": "{child_id}"
            }}
            "#
    ))
    .unwrap();

    let parent_id = new_entity(&tx, &parent_schema, parent_data).unwrap();

    let child_request = EntityRequest {
        0: vec![EntityField::Attribute(child_attr.clone())],
    };

    let request = EntityRequest {
        0: vec![EntityField::Entity(EntityAttribute {
            attribute: reference_attr.clone(),
            request: child_request,
        })],
    };

    let result = get(&tx, &parent_id, request).unwrap();

    let expected_child = result.get(&reference_attr.to_string()).unwrap();
    assert!(matches!(expected_child, Value::Object(..)));
}
