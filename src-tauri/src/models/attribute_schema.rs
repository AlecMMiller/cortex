use serde::{Deserialize, Serialize};
use specta::Type;

use crate::macros::macros::create_id;

use super::{
    attribute_type::{AttributeType, CreateAttributeType},
    entity_schema::EntitySchemaId,
};

create_id!(AttributeSchemaId);

#[derive(Type, Serialize)]
pub struct AttributeSchema {
    pub id: AttributeSchemaId,
    pub name: String,
    pub attr_type: AttributeType,
    pub quantity: Quantity,
}

#[derive(Type, Deserialize)]
pub struct CreateAttributeSchema {
    pub entity: EntitySchemaId,
    pub name: String,
    pub attr_type: CreateAttributeType,
    pub quantity: Quantity,
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
pub enum Quantity {
    Optional,
    Required,
    List,
}
