use std::collections::{HashMap, HashSet};

use rusqlite::{Error, Result, Transaction};
use serde::Deserialize;
use serde_json::{Map, Value};
use specta::Type;

use crate::macros::macros::create_id;

use super::{
    attribute_getters::{get_reference_attrs, get_text_attrs},
    attribute_schema::{AttributeSchemaId, Quantity, RawAttributeSchema, SchemaMap},
    attribute_type::{AttributeType, SimpleAttributeType},
    entity_schema::EntitySchemaId,
    response_map::EntitiesData,
};

create_id!(EntityId);

pub fn new_entity(tx: &Transaction, schema_id: &EntitySchemaId, data: Value) -> Result<EntityId> {
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

#[derive(Deserialize, Type)]
pub enum EntityField {
    Entity(EntityAttribute),
    Attribute(AttributeSchemaId),
}

#[derive(Deserialize, Type)]
pub struct EntityAttribute {
    pub attribute: AttributeSchemaId,
    pub request: EntityRequest,
}

#[derive(Deserialize, Type)]
pub struct EntityRequest(pub Vec<EntityField>);

pub type EntityResponse = Map<String, Value>;

struct RequestPlan<'a> {
    entities: &'a Vec<&'a EntityId>,
    text: HashSet<&'a AttributeSchemaId>,
}

impl<'a> RequestPlan<'a> {
    pub fn new(entities: &'a Vec<&'a EntityId>) -> Self {
        Self {
            entities,
            text: HashSet::new(),
        }
    }

    pub fn execute(self, tx: &Transaction) -> Result<EntitiesData> {
        let mut response_map = None;

        let text_attrs: Vec<&AttributeSchemaId> = self.text.into_iter().collect();
        response_map = get_text_attrs(tx, response_map, self.entities, &text_attrs)?;

        match response_map {
            Some(b) => Ok(b.finalize()),
            None => Ok(HashMap::new()),
        }
    }

    pub fn add_attr(&mut self, schema: &SchemaMap, attribute: &'a EntityField) -> Result<()> {
        match attribute {
            EntityField::Entity(..) => {}
            EntityField::Attribute(attribute) => {
                let schema_entry = schema.get(&attribute);
                let schema_entry = match schema_entry {
                    Some(entry) => Ok(entry),
                    None => Err(Error::ModuleError("Schema entry not found".to_string())),
                }?;

                match schema_entry.attr_type {
                    AttributeType::ReferenceAttribute(..) => Err(Error::InvalidQuery),
                    AttributeType::SimpleAttributeType(attr_type) => {
                        match attr_type {
                            SimpleAttributeType::Text | SimpleAttributeType::RichText => {
                                self.text.insert(attribute);
                            }
                        }
                        Ok(())
                    }
                }?;
            }
        }
        Ok(())
    }
}

fn get_many<'a>(
    tx: &Transaction,
    entity_ids: Vec<&'a EntityId>,
    request: &EntityRequest,
) -> Result<HashMap<EntityId, EntityResponse>> {
    let mut result: HashMap<EntityId, EntityResponse> = HashMap::new();

    if entity_ids.len() == 0 {
        return Ok(result);
    }

    let request = &request.0;
    let schema = RawAttributeSchema::get_for_entity(tx, entity_ids[0])?;

    // Get child attributes
    for attr in request {
        match attr {
            EntityField::Attribute(..) => {}
            EntityField::Entity(entity_request) => {
                let attr = &entity_request.attribute;
                let subrequest = &entity_request.request;

                let schema_info = schema.get(attr).unwrap();
                let quantity = &schema_info.quantity;

                let mut data = get_reference_attrs(tx, &entity_ids, attr)?;

                let mut children = Vec::new();

                for datum in data.values() {
                    children.extend(datum);
                }

                let child_data = get_many(tx, children, subrequest)?;

                for (entity_id, children) in data.drain().take(1) {
                    let entity_map = result.entry(entity_id).or_default();

                    let data = match quantity {
                        Quantity::Required => {
                            if children.len() != 1 {
                                todo!();
                            }

                            Value::Object(child_data.get(&children[0]).unwrap().clone())
                        }
                        _ => todo!(),
                    };

                    entity_map.insert(attr.to_string(), data);
                }
            }
        }
    }

    let mut plan = RequestPlan::new(&entity_ids);

    for attr in request {
        plan.add_attr(&schema, attr)?;
    }

    let mut data = plan.execute(tx)?;

    for entity_id in entity_ids {
        let mut entity_data = data.remove(entity_id).unwrap_or(HashMap::new());

        let entity_map = result.entry(entity_id.clone()).or_default();

        for attr in request {
            match attr {
                EntityField::Attribute(attribute) => {
                    let schema = schema.get(&attribute);
                    let schema = match schema {
                        Some(schema) => schema,
                        None => continue,
                    };

                    let attr_data = entity_data.remove(&attribute);
                    let quantity = &schema.quantity;

                    let data = match quantity {
                        Quantity::Required => match attr_data {
                            Some(mut data) => {
                                if data.len() > 1 {
                                    todo!()
                                } else {
                                    Ok(data.remove(0))
                                }
                            }
                            None => Err(Error::QueryReturnedNoRows),
                        },
                        Quantity::Optional => todo!(),
                        Quantity::List => match attr_data {
                            Some(data) => Ok(Value::Array(data)),
                            None => Ok(Value::Array(Vec::new())),
                        },
                    }?;

                    entity_map.insert(attribute.to_string(), data);
                }
                EntityField::Entity(..) => (),
            }
        }
    }

    Ok(result)
}

pub fn get(
    tx: &Transaction,
    entity_id: &EntityId,
    request: EntityRequest,
) -> Result<EntityResponse> {
    let mut result = get_many(tx, vec![entity_id], &request)?;

    let result = result.drain().next();

    match result {
        Some((_k, v)) => Ok(v),
        None => Err(Error::InvalidQuery),
    }
}
