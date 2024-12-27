pub mod add_entity;
mod delete_entity;
pub mod get_entity;
pub use add_entity::add_entity;
pub use get_entity::get;

use serde::Deserialize;
use serde_json::{Map, Value};
use specta::Type;

use crate::models::attribute_schema::AttributeSchemaId;

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
