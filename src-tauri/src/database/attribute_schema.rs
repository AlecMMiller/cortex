use crate::macros::macros::create_id;
use rusqlite::{
    params,
    types::{FromSql, FromSqlError},
    Result, ToSql, Transaction,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    attribute_type::{AttributeType, CreateAttributeType},
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
        attribute_type::{CreateReferenceAttribute, ReferenceAttribute, SimpleAttributeType},
        entity_schema::{CreateEntitySchema, EntitySchema},
        test::test_util::setup,
    };

    use super::*;

    #[test]
    fn new() {
        let mut conn = setup();
        let tx = conn.transaction().unwrap();
        let entity_name = "Foo";
        let attribute_name = "Bar";

        let entity = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: entity_name.to_string(),
            },
        )
        .expect("Unable to create entity");
        let entity_id = entity.id;

        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity: entity_id,
                name: attribute_name.to_string(),
                quantity: Quantity::Required,
                attr_type: CreateAttributeType::SimpleAttributeType(SimpleAttributeType::Text),
            },
        )
        .expect("Failed to create attribute");
        let attribute_id = new_attribute.id;

        let stored = AttributeSchema::get(&tx, &attribute_id).expect("Failed to get stored");

        assert_eq!(stored.id, attribute_id);
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
        let attribute_name = "Bar";

        let entity = EntitySchema::new(
            &tx,
            CreateEntitySchema {
                name: entity_name.to_string(),
            },
        )
        .expect("Unable to create entity");
        let entity_id = entity.id;

        let new_attribute = AttributeSchema::new(
            &tx,
            CreateAttributeSchema {
                entity: entity_id.clone(),
                name: attribute_name.to_string(),
                quantity: Quantity::Required,
                attr_type: CreateAttributeType::CreateReferenceAttribute(
                    CreateReferenceAttribute {
                        id: entity_id.clone(),
                    },
                ),
            },
        )
        .expect("Failed to create attribute");
        let attribute_id = new_attribute.id;

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
