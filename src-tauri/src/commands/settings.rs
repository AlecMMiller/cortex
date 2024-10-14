use crate::commands::Error;
use crate::utils::get_connection;
use crate::{models::settings::Setting, PoolWrapper};
use tauri::State;

#[tauri::command]
pub fn get_setting<'a>(state: State<'_, PoolWrapper>, key: &str) -> Result<Setting, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Setting::get(&mut conn, key)?)
}

#[tauri::command]
pub fn get_setting_or_set<'a>(
    state: State<'_, PoolWrapper>,
    key: &str,
    value: &str,
) -> Result<Setting, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Setting::get_or_set(&mut conn, key, value)?)
}

#[tauri::command]
pub fn update_setting<'a>(
    state: State<'_, PoolWrapper>,
    key: &str,
    value: &str,
) -> Result<(), Error> {
    let mut conn = get_connection(state.pool.clone());
    println!("Set setting {key} to {value}");
    Ok(Setting::set(&mut conn, key, value)?)
}
