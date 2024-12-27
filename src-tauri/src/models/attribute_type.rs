use rusqlite::types::{FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::macros::macros::create_id;

use super::entity_schema::EntitySchemaId;

create_id!(ReferenceAttributeId);
create_id!(TextAttributeId);

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum AttributeType {
    SimpleAttributeType(SimpleAttributeType),
    ReferenceAttribute(ReferenceAttribute),
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq, Clone, Copy)]
pub enum SimpleAttributeType {
    Text,
    RichText,
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub struct ReferenceAttribute {
    pub id: EntitySchemaId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum CreateAttributeType {
    SimpleAttributeType(SimpleAttributeType),
    CreateReferenceAttribute(CreateReferenceAttribute),
}

#[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
pub struct CreateReferenceAttribute {
    pub id: EntitySchemaId,
}

impl AttributeType {
    pub fn columns_result(
        type_column: ValueRef<'_>,
        id_column: ValueRef<'_>,
        name_column: ValueRef<'_>,
    ) -> FromSqlResult<Self> {
        let value = type_column.as_str()?;
        match value {
            "Reference" => {
                let name = name_column.as_str()?;
                let id = EntitySchemaId::column_result_manual(id_column)?;

                let reference = ReferenceAttribute {
                    id,
                    name: name.into(),
                };

                Ok(AttributeType::ReferenceAttribute(reference))
            }
            simple => Ok(AttributeType::SimpleAttributeType(
                SimpleAttributeType::from_sql(simple)?,
            )),
        }
    }
}
