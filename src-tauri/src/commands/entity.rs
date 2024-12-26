use super::Error;
use crate::{
    database::{
        entity::{add_entity, get, EntityRequest, EntityResponse},
        entity_schema::EntitySchemaId,
    },
    models::entity::EntityId,
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
    let new = add_entity(&tx, &schema, data)?;
    tx.commit()?;
    Ok(new)
}

#[tauri::command]
#[specta::specta]
pub fn get_entity(
    pool_wrapper: State<'_, PoolWrapper>,
    entity: EntityId,
    request: EntityRequest,
) -> Result<EntityResponse, Error> {
    let mut conn = pool_wrapper.pool.get()?;
    let tx = conn.transaction()?;
    let entity = get(&tx, &entity, &request)?;
    tx.commit()?;
    Ok(entity)
}
