use super::Error;
use crate::{
    database::{
        entity::{new_entity, EntityId},
        entity_schema::EntitySchemaId,
    },
    setup::PoolWrapper,
};
use serde_json::Value;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub fn create_entity(
    pool_wrapper: State<'_, PoolWrapper>,
    schema: EntitySchemaId,
    data: Value,
) -> Result<EntityId, Error> {
    let mut conn = pool_wrapper.pool.get()?;
    let tx = conn.transaction()?;
    let new = new_entity(&tx, &schema, data)?;
    tx.commit()?;
    Ok(new)
}
