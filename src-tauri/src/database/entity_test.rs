use serde_json::Value;

use crate::{
    database::{
        entity::{get, EntityField, EntityRequest},
        test::test_util::{assert_string_key, setup, ASD, ESD, RSD},
    },
    models::attribute_schema::Quantity,
};

use super::entity::{add_entity, EntityAttribute};

#[test]
fn text() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();
    let schema_id = &ESD::default().create(&tx);
    let attribute_id = ASD::default().create(&tx, schema_id);

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_id}": "Hello world" 
            }}
            "#
    ))
    .unwrap();

    let entity_id = add_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![EntityField::Attribute(attribute_id.clone())],
    };

    let result = get(&tx, &entity_id, &request).unwrap();

    assert_string_key(&result, attribute_id, "Hello world");
}

#[test]
fn list() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();

    let schema_id = &ESD::default().create(&tx);
    let attribute_id = ASD::default()
        .quantity(Quantity::List)
        .create(&tx, schema_id);

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_id}": ["Hello world", "Hello moon"] 
            }}
            "#
    ))
    .unwrap();

    let entity_id = add_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![EntityField::Attribute(attribute_id.clone())],
    };

    let result = get(&tx, &entity_id, &request).unwrap();
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

    let schema_id = &ESD::create_default(&tx);

    let attribute_1_id = ASD::create_default(&tx, schema_id);
    let attribute_2_id = ASD::default().name("2").create(&tx, schema_id);

    let data = serde_json::from_str(&format!(
        r#"
            {{
              "{attribute_1_id}": "Message 1",
              "{attribute_2_id}": "Message 2"
            }}
            "#
    ))
    .unwrap();

    let entity_id = add_entity(&tx, &schema_id, data).unwrap();

    let request = EntityRequest {
        0: vec![
            EntityField::Attribute(attribute_1_id.clone()),
            EntityField::Attribute(attribute_2_id.clone()),
        ],
    };

    let result = get(&tx, &entity_id, &request).unwrap();

    assert_string_key(&result, attribute_1_id, "Message 1");
    assert_string_key(&result, attribute_2_id, "Message 2");
}

#[test]
fn reference() {
    let mut conn = setup();
    let tx = conn.transaction().unwrap();

    let parent_schema = &ESD::create_default(&tx);
    let child_schema = &ESD {
        name: "Child".to_string(),
    }
    .create(&tx);

    let reference_attr = RSD::create_default(&tx, parent_schema, child_schema);

    let child_attr = ASD::create_default(&tx, child_schema);

    let child_data = serde_json::from_str(&format!(
        r#"
            {{
              "{child_attr}": "Message 1"
            }}
            "#
    ))
    .unwrap();

    let child_id = add_entity(&tx, &child_schema, child_data).unwrap();

    let parent_data = serde_json::from_str(&format!(
        r#"
            {{
              "{reference_attr}": "{child_id}"
            }}
            "#
    ))
    .unwrap();

    let parent_id = add_entity(&tx, &parent_schema, parent_data).unwrap();

    let child_request = EntityRequest {
        0: vec![EntityField::Attribute(child_attr.clone())],
    };

    let request = EntityRequest {
        0: vec![EntityField::Entity(EntityAttribute {
            attribute: reference_attr.clone(),
            request: child_request,
        })],
    };

    let result = get(&tx, &parent_id, &request).unwrap();

    let expected_child = result.get(&reference_attr.to_string()).unwrap();
    assert!(matches!(expected_child, Value::Object(..)));
}
