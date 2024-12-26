use crate::models::{attribute_schema::AttributeSchemaId, entity::EntityId};
use serde_json::Value;
use std::collections::HashMap;

pub struct ResponseMap {
    current_entity: EntityId,
    current_attribute: AttributeSchemaId,
    current_vector: Vec<Value>,
    entities: EntitiesData,
}

pub type EntityData = HashMap<AttributeSchemaId, Vec<Value>>;
pub type EntitiesData = HashMap<EntityId, EntityData>;

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
