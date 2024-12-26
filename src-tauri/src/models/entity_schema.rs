use serde::Serialize;
use specta::Type;

use super::attribute_schema::AttributeSchema;
use crate::macros::macros::create_id;

create_id!(EntitySchemaId);

#[derive(Type, Serialize)]
pub struct EntitySchema {
    pub id: EntitySchemaId,
    pub name: String,
    pub attributes: Vec<AttributeSchema>,
}

pub struct CreateEntitySchema {
    pub name: String,
}
