use std::collections::HashMap;

use rusqlite::{params_from_iter, ParamsFromIter, Statement, ToSql, Transaction};
use serde_json::Value;

use super::{attribute_schema::AttributeSchemaId, entity::EntityId, response_map::ResponseMap};

fn build_question_marks(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}

fn build_request(attr_table: &str, num_entities: usize, num_attrs: usize) -> String {
    let entity_part = build_question_marks(num_entities);
    let attr_part = build_question_marks(num_attrs);
    format!(
        "SELECT a.entity, a.schema, a.value FROM {attr_table} a LEFT JOIN entity e on a.entity = e.id WHERE e.id IN ({entity_part}) AND a.schema IN ({attr_part}) ORDER BY a.entity, a.schema, a.value"
    )
}

fn prepare<'a>(
    tx: &'a Transaction,
    attr_table: &str,
    num_entities: usize,
    num_attrs: usize,
) -> rusqlite::Result<Statement<'a>> {
    tx.prepare(&build_request(attr_table, num_entities, num_attrs))
}

fn get_params<'a>(
    entities: &'a Vec<&EntityId>,
    attrs: &'a Vec<&AttributeSchemaId>,
) -> ParamsFromIter<Vec<&'a (dyn ToSql)>> {
    assert_ne!(entities.len(), 0);
    assert_ne!(attrs.len(), 0);

    let mut params: Vec<&(dyn ToSql)> = Vec::new();

    for entity in entities {
        params.push(entity);
    }

    for attr in attrs {
        params.push(attr);
    }

    params_from_iter(params)
}

pub fn get_text_attrs(
    tx: &Transaction,
    mut map: Option<ResponseMap>,
    entities: &Vec<&EntityId>,
    attrs: &Vec<&AttributeSchemaId>,
) -> rusqlite::Result<Option<ResponseMap>> {
    assert_ne!(entities.len(), 0);

    if attrs.len() == 0 {
        return Ok(map);
    }

    let mut statement = prepare(tx, "text_attribute", entities.len(), attrs.len())?;
    let params = get_params(entities, attrs);
    let mut rows = statement.query(params)?;

    while let Some(row) = rows.next()? {
        let entity: EntityId = row.get(0)?;
        let attribute = row.get(1)?;
        let value = Value::String(row.get(2)?);

        map = ResponseMap::add(map, entity, attribute, value);
    }

    Ok(map)
}

fn build_ref_request(num_entities: usize) -> String {
    let entity_part = build_question_marks(num_entities);
    format!(
        "SELECT a.entity, a.value FROM reference_attribute a LEFT JOIN entity e on a.entity = e.id WHERE e.id IN ({entity_part}) AND a.schema=? ORDER BY a.entity"
    )
}

fn get_ref_params<'a>(
    entities: &'a Vec<&EntityId>,
    attr: &'a AttributeSchemaId,
) -> ParamsFromIter<Vec<&'a (dyn ToSql)>> {
    assert_ne!(entities.len(), 0);

    let mut params: Vec<&(dyn ToSql)> = Vec::new();

    for entity in entities {
        params.push(entity);
    }

    params.push(attr);

    params_from_iter(params)
}

pub fn get_reference_attrs(
    tx: &Transaction,
    entities: &Vec<&EntityId>,
    attr: &AttributeSchemaId,
) -> rusqlite::Result<HashMap<EntityId, Vec<EntityId>>> {
    assert_ne!(entities.len(), 0);

    let mut statement = tx.prepare(&build_ref_request(entities.len()))?;

    let params = get_ref_params(entities, attr);
    let mut rows = statement.query(params)?;

    let mut map = HashMap::new();

    while let Some(row) = rows.next()? {
        let parent_entity: EntityId = row.get(0)?;
        let value: EntityId = row.get(1)?;

        let parent: &mut Vec<EntityId> = map.entry(parent_entity).or_default();

        parent.push(value);
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::database::{
        add_entity::new_entity,
        attribute_schema::{AttributeSchema, CreateAttributeSchema, Quantity},
        attribute_type::{CreateAttributeType, CreateReferenceAttribute},
        entity_schema::{CreateEntitySchema, EntitySchema},
        test::test_util::setup,
    };

    use super::*;

    #[test]
    fn build_simple_request() {
        let result = build_request("text_attribute", 1, 1);

        assert_eq!(result, "SELECT a.entity, a.schema, a.value FROM text_attribute a LEFT JOIN entity e on a.entity = e.id WHERE e.id IN (?) AND a.schema IN (?) ORDER BY a.entity, a.schema, a.value");
    }

    #[test]
    fn build_multi_request() {
        let result = build_request("text_attribute", 3, 2);

        assert_eq!(result, "SELECT a.entity, a.schema, a.value FROM text_attribute a LEFT JOIN entity e on a.entity = e.id WHERE e.id IN (?,?,?) AND a.schema IN (?,?) ORDER BY a.entity, a.schema, a.value");
    }

    #[test]
    fn get_refs() {
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
                attr_type: CreateAttributeType::CreateReferenceAttribute(
                    CreateReferenceAttribute {
                        id: child_schema.clone(),
                    },
                ),
            },
        )
        .unwrap()
        .id;

        let child_data = serde_json::from_str(&format!(
            r#"
            {{
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

        let result = get_reference_attrs(&tx, &vec![&parent_id], &reference_attr).unwrap();

        let children = result.get(&parent_id).unwrap();

        assert_eq!(children.len(), 1);
        let child = children.get(0).unwrap();
        assert_eq!(child, &child_id);
    }
}
