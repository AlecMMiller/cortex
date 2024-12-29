use serde::Serialize;

use super::attribute_schema::AttributeSchema;
use crate::macros::macros::create_id;

create_id!(EntitySchemaId);

#[derive(Serialize)]
pub struct EntitySchema {
    pub id: EntitySchemaId,
    pub name: String,
    pub attributes: Vec<AttributeSchema>,
}

pub struct CreateEntitySchema {
    pub name: String,
}
