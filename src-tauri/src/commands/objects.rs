use crate::{
    models::objects::{SchemaDefinition, SchemaId, SchemaWithProperties},
    setup::PoolWrapper,
    utils::get_connection,
};
use tauri::State;

use super::Error;

#[tauri::command]
#[specta::specta]
pub fn get_all_schemas<'a>(state: State<'_, PoolWrapper>) -> Result<Vec<SchemaDefinition>, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(SchemaDefinition::get_all(&mut conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_schema<'a>(uuid: SchemaId, state: State<'_, PoolWrapper>) -> Result<(), Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(SchemaWithProperties::get(&mut conn, &uuid)?)
}

#[tauri::command]
#[specta::specta]
pub fn rename_schema<'a>(
    uuid: SchemaId,
    name: &str,
    state: State<'_, PoolWrapper>,
) -> Result<(), Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(SchemaWithProperties::rename(&mut conn, &uuid, name)?)
}

#[tauri::command]
#[specta::specta]
pub fn create_schema<'a>(
    name: &str,
    state: State<'_, PoolWrapper>,
) -> Result<SchemaDefinition, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(SchemaDefinition::new(&mut conn, name)?)
}
