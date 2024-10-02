use crate::commands::Error;
use crate::db;
use crate::{models::settings::Setting, PoolWrapper};
use tauri::State;

#[tauri::command]
pub fn get_setting<'a>(state: State<'_, PoolWrapper>, key: &str) -> Result<Setting, Error> {
    Ok(db::settings::get(state.pool.clone(), key)?)
}

#[tauri::command]
pub fn get_setting_or_set<'a>(
    state: State<'_, PoolWrapper>,
    key: &str,
    value: &str,
) -> Result<Setting, Error> {
    Ok(db::settings::get_or_set(state.pool.clone(), key, value)?)
}
