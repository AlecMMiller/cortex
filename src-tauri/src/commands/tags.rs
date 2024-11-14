use crate::commands::Error;
use crate::models::notes::NoteId;
use crate::models::tags::Tag;
use crate::setup::PoolWrapper;
use crate::utils::get_connection;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub fn get_available_tags_containing<'a>(
    state: State<'_, PoolWrapper>,
    content: &str,
    max_results: i64,
    note_uuid: NoteId,
) -> Result<(Vec<Tag>, bool), Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Tag::get_available_containing(
        &mut conn,
        &content,
        max_results,
        note_uuid,
    )?)
}
