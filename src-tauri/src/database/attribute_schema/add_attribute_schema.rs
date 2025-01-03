use rusqlite::Transaction;

use crate::{
    database::New,
    models::attribute_schema::{AttributeSchema, AttributeSchemaId, CreateAttributeSchema},
    utils::get_timestamp,
};

impl New<CreateAttributeSchema> for AttributeSchema {
    fn new(tx: &Transaction, data: CreateAttributeSchema) -> rusqlite::Result<Self> {
        let reference = data.attr_type.get_ref();

        let new_attribute = Self {
            id: AttributeSchemaId::new(),
            name: data.name,
            quantity: data.quantity,
            attr_type: data.attr_type.get_full(tx)?,
        };

        let created_at = get_timestamp();

        tx.execute(
            "INSERT INTO attribute_schema (id, entity, name, type, reference, quantity, created, updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
            (
                &new_attribute.id,
                data.entity,
                &new_attribute.name,
                &data.attr_type,
                &reference,
                &new_attribute.quantity,
                created_at
            ),
        )?;

        Ok(new_attribute)
    }
}
