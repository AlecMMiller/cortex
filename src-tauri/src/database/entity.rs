use std::collections::{HashMap, HashSet};

use rusqlite::{params, Error, Result, Transaction};
use serde::Deserialize;
use serde_json::{Map, Value};
use specta::Type;

use crate::macros::macros::create_id;

use super::{
    attribute_schema::{AttributeSchemaId, RawAttributeSchema, SchemaMap},
    attribute_type::{AttributeType, SimpleAttributeType},
    entity_schema::EntitySchemaId,
};

create_id!(EntityId);

pub fn new_entity(tx: &Transaction, schema_id: &EntitySchemaId, data: Value) -> Result<EntityId> {
    let data = match data {
        Value::Object(obj) => Ok(obj),
        _ => Err(Error::InvalidQuery),
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
            Err(_) => Err(Error::InvalidQuery),
        }?;

        let schema_entry = match schema.get(&key) {
            Some(entry) => Ok(entry),
            None => Err(Error::InvalidQuery),
        }?;

        match value {
            Value::String(val) => schema_entry.insert_string(tx, &id, &val),
            _ => Err(Error::InvalidQuery),
        }?;
    }

    Ok(id)
}

#[derive(Deserialize, Type)]
pub enum EntityField {
    Entity(EntityRequest),
    Attribute(AttributeSchemaId),
}

#[derive(Deserialize, Type)]
pub struct EntityRequest(Vec<EntityField>);

pub type EntityResponse = Map<String, Value>;

struct IndividualRequests<'a> {
    pub entities: HashSet<&'a EntityId>,
    pub attributes: HashSet<&'a AttributeSchemaId>,
}

impl<'a> IndividualRequests<'a> {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
            attributes: HashSet::new(),
        }
    }
}

struct RequestPlan<'a> {
    text: IndividualRequests<'a>,
}

struct ResponseMap {
    current_entity: EntityId,
    current_attribute: AttributeSchemaId,
    current_vector: Vec<Value>,
    entities: EntitiesData,
}

type EntityData = HashMap<AttributeSchemaId, Vec<Value>>;
type EntitiesData = HashMap<EntityId, EntityData>;

impl ResponseMap {
    fn new(first_entity: EntityId, first_attribute: AttributeSchemaId, first_value: Value) -> Self {
        let mut vector = Vec::new();
        let entities: EntitiesData = HashMap::new();

        vector.push(first_value);

        Self {
            current_entity: first_entity,
            current_attribute: first_attribute,
            current_vector: vector,
            entities,
        }
    }

    fn push_current(mut self) -> Self {
        let entity = self
            .entities
            .entry(self.current_entity.clone())
            .or_insert(HashMap::new());

        entity.insert(self.current_attribute.clone(), self.current_vector);
        self.current_vector = Vec::new();
        self
    }

    fn change_attribute(mut self, attribute: AttributeSchemaId) -> Self {
        self = self.push_current();
        self.current_attribute = attribute;
        self
    }

    fn change_entity(mut self, entity: EntityId, attribute: AttributeSchemaId) -> Self {
        self = self.change_attribute(attribute);
        self.current_entity = entity;
        self
    }

    fn add_value(mut self, entity: EntityId, attribute: AttributeSchemaId, value: Value) -> Self {
        if entity != self.current_entity {
            self = self.change_entity(entity, attribute)
        } else if attribute != self.current_attribute {
            self = self.change_attribute(attribute);
        }

        self.current_vector.push(value);
        self
    }

    pub fn finalize(self) -> EntitiesData {
        self.push_current().entities
    }

    pub fn add(
        builder: Option<Self>,
        entity: EntityId,
        attribute: AttributeSchemaId,
        value: Value,
    ) -> Option<Self> {
        match builder {
            Some(b) => Some(b.add_value(entity, attribute, value)),
            None => Some(Self::new(entity, attribute, value)),
        }
    }
}

impl<'a> RequestPlan<'a> {
    pub fn new() -> Self {
        Self {
            text: IndividualRequests::new(),
        }
    }

    pub fn execute(self, tx: &Transaction) -> Result<EntitiesData> {
        let mut response_map = None;

        let mut text_statement = tx.prepare(
            "SELECT 
                    a.entity, a.schema, a.value 
                  FROM text_attribute a LEFT JOIN entity e ON a.entity = e.id 
                  WHERE e.id=?1",
        )?;
        let text_entities: Vec<&EntityId> = self.text.entities.into_iter().collect();
        let mut rows = text_statement.query(params![text_entities[0]])?;

        while let Some(row) = rows.next()? {
            let entity: EntityId = row.get(0)?;
            let attribute = row.get(1)?;
            let value = Value::String(row.get(2)?);

            response_map = ResponseMap::add(response_map, entity, attribute, value);
        }

        match response_map {
            Some(b) => Ok(b.finalize()),
            None => Err(Error::InvalidQuery),
        }
    }

    pub fn add_attr(
        &mut self,
        schema: &SchemaMap,
        entity: &'a EntityId,
        attribute: &'a EntityField,
    ) -> Result<()> {
        match attribute {
            EntityField::Entity(..) => {}
            EntityField::Attribute(attribute) => {
                let schema_entry = schema.get(&attribute);
                let schema_entry = match schema_entry {
                    Some(entry) => Ok(entry),
                    None => Err(Error::InvalidQuery),
                }?;

                match schema_entry.attr_type {
                    AttributeType::ReferenceAttribute(..) => Err(Error::InvalidQuery),
                    AttributeType::SimpleAttributeType(attr_type) => {
                        match attr_type {
                            SimpleAttributeType::Text | SimpleAttributeType::RichText => {
                                self.text.entities.insert(entity);
                                self.text.attributes.insert(attribute);
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

pub fn get(
    tx: &Transaction,
    entity_id: &EntityId,
    request: EntityRequest,
) -> Result<EntityResponse> {
    let request = request.0;
    let schema = RawAttributeSchema::get_for_entity(tx, entity_id)?;

    let mut plan = RequestPlan::new();

    for attr in &request {
        plan.add_attr(&schema, entity_id, attr)?;
    }

    let mut data = plan.execute(tx)?;

    let mut entity_data = data.remove(entity_id).unwrap();

    let mut entity_map = Map::new();

    for attr in request {
        match attr {
            EntityField::Attribute(attribute) => {
                let attr_data = entity_data.remove(&attribute);
                let mut attr_data = match attr_data {
                    Some(data) => data,
                    None => Vec::new(),
                };
                let first = attr_data.remove(0);
                entity_map.insert(attribute.to_string(), first);
            }
            EntityField::Entity(..) => (),
        }
    }

    Ok(entity_map)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::database::{
        attribute_type::SimpleAttributeType,
        entity::EntityRequest,
        test::test_util::{create_attribute_schema, create_entity_schema, setup},
    };

    use super::{get, EntityField};

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let schema_id = create_entity_schema(&tx);
        let attribute_id =
            create_attribute_schema(&tx, "Bar", schema_id.clone(), SimpleAttributeType::Text);

        let data = serde_json::from_str(&format!(
            r#"
            {{
              "{attribute_id}": "Hello world" 
            }}
            "#
        ))
        .unwrap();

        let entity_id = super::new_entity(&tx, &schema_id, data).unwrap();

        let request = EntityRequest {
            0: vec![EntityField::Attribute(attribute_id.clone())],
        };

        let result = get(&tx, &entity_id, request).unwrap();

        let val = result.get(&attribute_id.to_string()).unwrap();

        assert_eq!(val, &Value::String("Hello world".to_string()));
    }
}
