use std::collections::HashMap;

use rusqlite::{params, Transaction};

use crate::{
    database::{Get, GetMany},
    models::{
        attribute_schema::{AttributeSchema, AttributeSchemaId},
        attribute_type::AttributeType,
        entity::EntityId,
        entity_schema::EntitySchemaId,
    },
};

use super::{RawAttributeSchema, SchemaMap};

impl Get<AttributeSchemaId> for RawAttributeSchema {
    fn get(tx: &Transaction, id: &AttributeSchemaId) -> rusqlite::Result<Self> {
        tx.query_row(
            "SELECT 
                    a.id, a.quantity, a.type, e.id, e.name 
                  FROM attribute_schema a LEFT JOIN entity_schema e ON a.reference = e.id 
                  WHERE a.id=?1",
            params![id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    quantity: row.get(1)?,
                    attr_type: AttributeType::columns_result(
                        row.get_ref(2)?,
                        row.get_ref(3)?,
                        row.get_ref(4)?,
                    )?,
                })
            },
        )
    }
}

impl Get<AttributeSchemaId> for AttributeSchema {
    fn get(tx: &Transaction, id: &AttributeSchemaId) -> rusqlite::Result<Self> {
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
}

impl GetMany<EntitySchemaId> for AttributeSchema {
    fn get_many(tx: &Transaction, id: &EntitySchemaId) -> rusqlite::Result<Vec<Self>> {
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

pub trait GetSchemaMap<T> {
    fn get_map(tx: &Transaction, data: &T) -> rusqlite::Result<HashMap<AttributeSchemaId, Self>>
    where
        Self: Sized;
}

impl GetSchemaMap<EntitySchemaId> for RawAttributeSchema {
    fn get_map(tx: &Transaction, id: &EntitySchemaId) -> rusqlite::Result<SchemaMap> {
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
}

impl GetSchemaMap<EntityId> for RawAttributeSchema {
    fn get_map(
        tx: &Transaction,
        id: &EntityId,
    ) -> rusqlite::Result<HashMap<AttributeSchemaId, Self>> {
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
}
