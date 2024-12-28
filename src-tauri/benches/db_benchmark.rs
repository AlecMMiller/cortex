use std::fs::remove_file;

use cortex::{
    database::{
        entity::{add_entity, get, EntityAttribute, EntityField, EntityRequest},
        migration::migrate,
        New,
    },
    models::{
        attribute_schema::{AttributeSchema, CreateAttributeSchema, Quantity},
        attribute_type::{CreateAttributeType, CreateReferenceAttribute, SimpleAttributeType},
        entity::EntityId,
        entity_schema::{CreateEntitySchema, EntitySchema},
    },
};
use rusqlite::{Connection, DatabaseName};

use criterion::{criterion_group, criterion_main, Criterion};

fn db_benchmark(c: &mut Criterion) {
    let _ = remove_file("bench.db");
    let mut conn = Connection::open("bench.db").unwrap();
    conn.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")
        .unwrap();
    conn.pragma_update(Some(DatabaseName::Main), "synchronous", "normal")
        .unwrap();

    let tx = conn.transaction().unwrap();

    migrate(&tx).unwrap();
    tx.commit().unwrap();

    let tx = conn.transaction().unwrap();
    let root_schema = EntitySchema::new(
        &tx,
        CreateEntitySchema {
            name: "Parent".to_string(),
        },
    )
    .unwrap();

    let attr1 = AttributeSchema::new(
        &tx,
        CreateAttributeSchema {
            entity: root_schema.id.clone(),
            name: "Attr1".to_string(),
            attr_type: CreateAttributeType::Simple(SimpleAttributeType::Text),
            quantity: Quantity::Required,
        },
    )
    .unwrap();

    let child_schema = EntitySchema::new(
        &tx,
        CreateEntitySchema {
            name: "Child".to_string(),
        },
    )
    .unwrap();

    let child_ref = AttributeSchema::new(
        &tx,
        CreateAttributeSchema {
            entity: root_schema.id.clone(),
            name: "Attr2".to_string(),
            quantity: Quantity::Required,
            attr_type: CreateAttributeType::Reference(CreateReferenceAttribute {
                id: child_schema.id.clone(),
            }),
        },
    )
    .unwrap();

    let grandchild_schema = EntitySchema::new(
        &tx,
        CreateEntitySchema {
            name: "Grandchild".to_string(),
        },
    )
    .unwrap();

    let grandchild_ref = AttributeSchema::new(
        &tx,
        CreateAttributeSchema {
            entity: child_schema.id.clone(),
            name: "Attr3".to_string(),
            quantity: Quantity::Required,
            attr_type: CreateAttributeType::Reference(CreateReferenceAttribute {
                id: grandchild_schema.id.clone(),
            }),
        },
    )
    .unwrap();

    let attr4 = AttributeSchema::new(
        &tx,
        CreateAttributeSchema {
            entity: grandchild_schema.id.clone(),
            name: "Attr4".to_string(),
            attr_type: CreateAttributeType::Simple(SimpleAttributeType::Text),
            quantity: Quantity::Required,
        },
    )
    .unwrap();

    tx.commit().unwrap();

    let attr1_id = attr1.id;
    let child_id = child_ref.id;
    let grandchild_id = grandchild_ref.id;
    let attr4_id = attr4.id;

    let mut to_get = Vec::new();

    let tx = conn.transaction().unwrap();

    for n in 0..100000 {
        let grandchild_data = serde_json::from_str(&format!(
            r#"
        {{
          "{attr4_id}": "Deeply nested text"
        }}
        "#
        ))
        .unwrap();
        let grandchild = add_entity(&tx, &grandchild_schema.id, grandchild_data).unwrap();

        let child_data = serde_json::from_str(&format!(
            r#"
        {{
          "{grandchild_id}": "{grandchild}"
        }}
        "#
        ))
        .unwrap();
        let child = add_entity(&tx, &child_schema.id, child_data).unwrap();

        let root_data = serde_json::from_str(&format!(
            r#"
        {{
          "{child_id}": "{child}",
          "{attr1_id}": "More text"
        }}
        "#
        ))
        .unwrap();
        let id = add_entity(&tx, &root_schema.id, root_data).unwrap();

        if n % 300 == 0 {
            to_get.push(id);
        }
    }

    tx.commit().unwrap();

    let grandchild_request = EntityRequest {
        0: vec![EntityField::Attribute(attr4_id)],
    };

    let child_request = EntityRequest {
        0: vec![EntityField::Entity(EntityAttribute {
            attribute: grandchild_id,
            request: grandchild_request,
        })],
    };

    let request = EntityRequest {
        0: vec![EntityField::Entity(EntityAttribute {
            attribute: child_id,
            request: child_request,
        })],
    };

    conn.pragma_update(Some(DatabaseName::Main), "optimize", "")
        .unwrap();

    let mut idx = 0;

    c.bench_function("db reader", |b| {
        b.iter(|| getter(&mut conn, &to_get, &request, &mut idx))
    });

    //remove_file("bench.db").unwrap();
}

fn getter(conn: &mut Connection, ids: &Vec<EntityId>, request: &EntityRequest, idx: &mut usize) {
    let to_use = *idx % ids.len();
    let id = &ids[to_use];

    let tx = conn.transaction().unwrap();

    get(&tx, id, request).unwrap();
    tx.commit().unwrap();

    *idx += 1;
}

criterion_group!(benches, db_benchmark);
criterion_main!(benches);
