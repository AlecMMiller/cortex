use tauri::State;

use crate::{
    models::objects::{Schema, SchemaId},
    setup::PoolWrapper,
    utils::get_connection,
};

use super::Error;

#[tauri::command]
#[specta::specta]
pub fn get_all_schemas<'a>(state: State<'_, PoolWrapper>) -> Result<Vec<Schema>, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Schema::get_all(&mut conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_schema<'a>(uuid: SchemaId, state: State<'_, PoolWrapper>) -> Result<Schema, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Schema::get(&mut conn, &uuid)?)
}

#[tauri::command]
#[specta::specta]
pub fn create_schema<'a>(name: &str, state: State<'_, PoolWrapper>) -> Result<Schema, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Schema::new(&mut conn, name)?)
}
