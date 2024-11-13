use crate::commands::Error;
use crate::models::notes::{Note, NoteId, NoteTitle};
use crate::models::tags::{NoteTag, Tag, TagId};
use crate::search::{search_by_content, search_by_title, TitleWithContext};
use crate::utils::get_connection;
use crate::WriterWrapper;
use crate::{PoolWrapper, SearcherWrapper};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub fn update_note(
    pool_wrapper: State<'_, PoolWrapper>,
    index_wrapper: State<'_, WriterWrapper>,
    uuid: NoteId,
    body: &str,
) -> Result<(), Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    let index = index_wrapper.writer.clone();
    Ok(Note::update_body(&mut conn, index, &uuid, &body)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_all_notes<'a>(state: State<'_, PoolWrapper>) -> Result<Vec<NoteTitle>, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(NoteTitle::get_all(&mut conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_notes_by_title<'a>(
    state: State<'_, SearcherWrapper>,
    title: &str,
    max_results: u16,
) -> Result<Vec<NoteTitle>, Error> {
    Ok(search_by_title(title, max_results, state.searcher.clone())?)
}

#[tauri::command]
#[specta::specta]
pub fn get_notes_by_content<'a>(
    state: State<'_, SearcherWrapper>,
    content: &str,
    max_results: u16,
    snippet_size: u16,
) -> Result<Vec<TitleWithContext>, Error> {
    Ok(search_by_content(
        content,
        max_results,
        snippet_size,
        state.searcher.clone(),
    )?)
}

#[tauri::command]
#[specta::specta]
pub fn get_note<'a>(state: State<'_, PoolWrapper>, uuid: NoteId) -> Result<Note, Error> {
    let mut conn = get_connection(state.pool.clone());
    Ok(Note::get(&mut conn, &uuid)?)
}

#[tauri::command]
#[specta::specta]
pub fn rename_note(
    pool_wrapper: State<'_, PoolWrapper>,
    writer_wrapper: State<'_, WriterWrapper>,
    uuid: NoteId,
    title: &str,
) -> Result<(), Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    Ok(Note::rename(
        &mut conn,
        writer_wrapper.writer.clone(),
        &uuid,
        &title,
    )?)
}

#[tauri::command]
#[specta::specta]
pub fn create_note(
    pool_wrapper: State<'_, PoolWrapper>,
    writer_wrapper: State<'_, WriterWrapper>,
    title: &str,
) -> Result<Note, Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    let writer = writer_wrapper.writer.clone();
    Ok(Note::new(&mut conn, writer, title, "")?)
}

#[tauri::command]
#[specta::specta]
pub fn get_direct_tags(
    pool_wrapper: State<'_, PoolWrapper>,
    uuid: NoteId,
) -> Result<Vec<Tag>, Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    Ok(Tag::get_direct_by_note(&mut conn, &uuid)?)
}

#[tauri::command]
#[specta::specta]
pub fn add_new_tag(
    pool_wrapper: State<'_, PoolWrapper>,
    uuid: NoteId,
    tag_text: &str,
) -> Result<(), Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    let tag = Tag::new(&mut conn, &tag_text)?;
    NoteTag::new(&mut conn, &uuid, &tag.uuid)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn add_tag(
    pool_wrapper: State<'_, PoolWrapper>,
    note_uuid: NoteId,
    tag_uuid: TagId,
) -> Result<(), Error> {
    let mut conn = get_connection(pool_wrapper.pool.clone());
    NoteTag::new(&mut conn, &note_uuid, &tag_uuid)?;
    Ok(())
}
