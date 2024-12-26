use std::collections::HashMap;

use crate::macros::macros::create_id;
use crate::models::entity::EntityId;
use rusqlite::{
    params,
    types::{FromSql, FromSqlError},
    Result, ToSql, Transaction,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;

use super::{
    attribute_type::{AttributeType, CreateAttributeType, SimpleAttributeType},
    entity_schema::EntitySchemaId,
};

create_id!(AttributeSchemaId);

#[derive(Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
pub enum Quantity {
    Optional,
    Required,
    List,
}

impl ToSql for Quantity {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Quantity::Optional => Ok("Optional".into()),
            Quantity::Required => Ok("Required".into()),
            Quantity::List => Ok("List".into()),
        }
    }
}

impl FromSql for Quantity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str()?;
        match value {
            "Optional" => Ok(Quantity::Optional),
            "Required" => Ok(Quantity::Required),
            "List" => Ok(Quantity::List),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

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

pub struct RawAttributeSchema {
    pub id: AttributeSchemaId,
    pub attr_type: AttributeType,
    pub quantity: Quantity,
}

pub type SchemaMap = HashMap<AttributeSchemaId, RawAttributeSchema>;

impl RawAttributeSchema {
    pub fn get_for_entity_schema(tx: &Transaction, id: &EntitySchemaId) -> Result<SchemaMap> {
        let mut statement = tx.prepare(
            "SELECT 
                    a.id, a.quantity, a.type, e.id, e.name 
                  FROM attribute_schema a LEFT JOIN entity_schema e ON a.reference = e.id 
                  WHERE a.entity=?1",
        )?;
        let mut rows = statement.query(params![id])?;

        let mut results = HashMap::new();
        while let Some(row) = rows.next()? {
            results.insert(
                row.get(0)?,
                RawAttributeSchema {
                    id: row.get(0)?,
                    quantity: row.get(1)?,
                    attr_type: AttributeType::columns_result(
                        row.get_ref(2)?,
                        row.get_ref(3)?,
                        row.get_ref(4)?,
                    )?,
                },
            );
        }

        Ok(results)
    }

    pub fn get_for_entity(
        tx: &Transaction,
        id: &EntityId,
    ) -> Result<HashMap<AttributeSchemaId, Self>> {
        let mut statement = tx.prepare(
            "SELECT 
                    a.id, a.quantity, a.type, e.id, e.name 
                  FROM entity ent
                  RIGHT JOIN entity_schema e on ent.schema = e.id
                  RIGHT JOIN attribute_schema a ON a.entity = e.id 
                  WHERE ent.id=?1",
        )?;
        let mut rows = statement.query(params![id])?;

        let mut results = HashMap::new();
        while let Some(row) = rows.next()? {
            results.insert(
                row.get(0)?,
                RawAttributeSchema {
                    id: row.get(0)?,
                    quantity: row.get(1)?,
                    attr_type: AttributeType::columns_result(
                        row.get_ref(2)?,
                        row.get_ref(3)?,
                        row.get_ref(4)?,
                    )?,
                },
            );
        }

        Ok(results)
    }

    pub fn insert_string(&self, tx: &Transaction, entity: &EntityId, val: &str) -> Result<()> {
        match &self.attr_type {
            AttributeType::ReferenceAttribute(reference) => {
                let target: EntityId = val.try_into().unwrap(); // TODO
                reference.insert_reference(tx, entity, &self.id, &target)
            }
            AttributeType::SimpleAttributeType(simple) => {
                simple.insert_string(tx, entity, &self.id, val)
            }
        }
    }

    pub fn insert_vec(&self, tx: &Transaction, entity: &EntityId, vals: &Vec<Value>) -> Result<()> {
        match self.attr_type {
            AttributeType::ReferenceAttribute(..) => todo!(),
            AttributeType::SimpleAttributeType(simple) => match simple {
                SimpleAttributeType::Text | SimpleAttributeType::RichText => {
                    simple.insert_string_vec(tx, entity, &self.id, vals)
                }
            },
        }
    }
}

impl AttributeSchema {
    pub fn new(tx: &Transaction, data: CreateAttributeSchema) -> Result<Self> {
        let reference = data.attr_type.get_ref();

        let new_attribute = Self {
            id: AttributeSchemaId::new(),
            name: data.name,
            quantity: data.quantity,
            attr_type: data.attr_type.get_full(tx)?,
        };

        tx.execute(
            "INSERT INTO attribute_schema (id, entity, name, type, reference, quantity) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &new_attribute.id,
                data.entity,
                &new_attribute.name,
                &data.attr_type,
                &reference,
                &new_attribute.quantity
            ),
        )?;

        Ok(new_attribute)
    }

    #[allow(dead_code)]
    fn get(tx: &Transaction, id: &AttributeSchemaId) -> Result<Self> {
        tx.query_row(
            "SELECT 
                    a.id, a.name, a.quantity, a.type, e.id, e.name 
                  FROM attribute_schema a LEFT JOIN entity_schema e ON a.reference = e.id 
                  WHERE a.id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    quantity: row.get(2)?,
                    attr_type: AttributeType::columns_result(
                        row.get_ref(3)?,
                        row.get_ref(4)?,
                        row.get_ref(5)?,
                    )?,
                })
            },
        )
    }

    pub fn get_for_entity(tx: &Transaction, id: &EntitySchemaId) -> Result<Vec<Self>> {
        let mut statement = tx.prepare(
            "SELECT 
                    a.id, a.name, a.quantity, a.type, e.id, e.name 
                  FROM attribute_schema a LEFT JOIN entity_schema e ON a.reference = e.id 
                  WHERE a.entity=?1",
        )?;
        let mut rows = statement.query(params![id])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                quantity: row.get(2)?,
                attr_type: AttributeType::columns_result(
                    row.get_ref(3)?,
                    row.get_ref(4)?,
                    row.get_ref(5)?,
                )?,
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{
        attribute_type::{ReferenceAttribute, SimpleAttributeType},
        test::test_util::{
            create_attribute_schema, create_entity_schema, create_reference_schema, setup, ASD,
            ESD, RSD,
        },
    };

    use super::*;

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_id = create_entity_schema(&tx, ESD::default());

        let attribute_name = "Bar";

        let new_attribute =
            create_attribute_schema(&tx, entity_id, ASD::default().name(attribute_name));

        let stored = AttributeSchema::get(&tx, &new_attribute).expect("Failed to get stored");

        assert_eq!(stored.id, new_attribute);
        assert_eq!(stored.name, attribute_name);
        assert_eq!(stored.quantity, Quantity::Required);
        assert_eq!(
            stored.attr_type,
            AttributeType::SimpleAttributeType(SimpleAttributeType::Text)
        );
    }

    #[test]
    fn reference() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_name = "Foo";

        let entity_id = create_entity_schema(&tx, ESD::default());
        let attribute_id =
            create_reference_schema(&tx, entity_id.clone(), entity_id.clone(), RSD::default());

        let stored = AttributeSchema::get(&tx, &attribute_id).expect("Failed to get stored");

        assert_eq!(
            stored.attr_type,
            AttributeType::ReferenceAttribute(ReferenceAttribute {
                id: entity_id,
                name: entity_name.to_string()
            })
        );
    }
}
