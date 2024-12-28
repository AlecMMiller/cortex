use std::collections::{HashMap, HashSet};

use rusqlite::{Error, Result, Transaction};
use serde_json::Value;

use crate::database::attribute::{get_reference_attrs, get_text_attrs};
use crate::models::attribute_type::{AttributeType, SimpleAttributeType};
use crate::models::entity::EntityId;
use crate::{
    database::{
        attribute_schema::{GetSchemaMap, RawAttributeSchema, SchemaMap},
        response_map::EntitiesData,
    },
    models::attribute_schema::{AttributeSchemaId, Quantity},
};

use super::{EntityField, EntityRequest, EntityResponse};

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
    let schema = RawAttributeSchema::get_map(tx, entity_ids[0])?;

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
        let mut entity_data = data.remove(entity_id).unwrap_or_default();

        let entity_map = result.entry(entity_id.clone()).or_default();

        for attr in request {
            match attr {
                EntityField::Attribute(attribute) => {
                    let Some(schema) = schema.get(attribute) else {
                        continue;
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
                        Quantity::List => Ok(Value::Array(attr_data.unwrap_or_default())),
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
    request: &EntityRequest,
) -> Result<EntityResponse> {
    let mut result = get_many(tx, vec![entity_id], &request)?;

    let result = result.drain().next();

    match result {
        Some((_k, v)) => Ok(v),
        None => Err(Error::InvalidQuery),
    }
}

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
                    AttributeType::Reference(..) => Err(Error::InvalidQuery),
                    AttributeType::Simple(attr_type) => {
                        match attr_type {
                            SimpleAttributeType::Longform => todo!(),
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
