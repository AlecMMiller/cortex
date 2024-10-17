use crate::commands::Error;
use crate::models::tags::Tag;
use crate::utils::get_connection;
use crate::PoolWrapper;
use tauri::State;

#[tauri::command]
pub fn get_tags_containing<'a>(
    state: State<'_, PoolWrapper>,
    content: &str,
    max_results: i64,
) -> Result<Vec<Tag>, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Tag::get_containing(&mut conn, &content, max_results)?)
}
