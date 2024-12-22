use tauri::State;

use super::Error;
use crate::database::entity_schema::{CreateEntitySchema, EntitySchema, EntitySchemaId};
use crate::setup::PoolWrapper;

#[tauri::command]
#[specta::specta]
pub fn create_entity_schema(
    pool_wrapper: State<'_, PoolWrapper>,
    name: String,
) -> Result<EntitySchema, Error> {
    let mut conn = pool_wrapper.pool.get()?;
    let tx = conn.transaction()?;
    let res = EntitySchema::new(&tx, CreateEntitySchema { name })?;
    tx.commit()?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub fn get_entity_schema(
    pool_wrapper: State<'_, PoolWrapper>,
    id: EntitySchemaId,
) -> Result<EntitySchema, Error> {
    let mut conn = pool_wrapper.pool.get()?;
    let tx = conn.transaction()?;
    let res = EntitySchema::get(&tx, &id)?;
    tx.commit()?;
    Ok(res)
}
