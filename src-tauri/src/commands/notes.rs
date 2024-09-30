use crate::commands::Error;
use crate::models::{Note, NoteId};
use crate::notes::NoteTitle;
use crate::search::search_by_title;
use crate::{notes, WriterWrapper};
use crate::{PoolWrapper, SearcherWrapper};
use tauri::State;

#[tauri::command]
pub fn update_note(
    pool_wrapper: State<'_, PoolWrapper>,
    index_wrapper: State<'_, WriterWrapper>,
    uuid: NoteId,
    body: &str,
) -> Result<(), ()> {
    let result = notes::update_body(
        pool_wrapper.pool.clone(),
        index_wrapper.writer.clone(),
        uuid,
        body,
    );
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[tauri::command]
pub fn get_last_updated_note<'a>(state: State<'_, PoolWrapper>) -> Result<String, ()> {
    let last_updated = notes::get_last_updated_or_create(state.pool.clone());
    match last_updated {
        Ok(note) => {
            let json = serde_json::to_string(&note);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[tauri::command]
pub fn get_all_notes<'a>(state: State<'_, PoolWrapper>) -> Result<Vec<NoteTitle>, Error> {
    Ok(notes::get_all_titles(state.pool.clone())?)
}

#[tauri::command]
pub fn get_notes_by_title<'a>(
    state: State<'_, SearcherWrapper>,
    title: &str,
) -> Result<Vec<NoteTitle>, Error> {
    Ok(search_by_title(title, 5, state.searcher.clone())?)
}

#[tauri::command]
pub fn get_note<'a>(state: State<'_, PoolWrapper>, uuid: NoteId) -> Result<Note, Error> {
    Ok(notes::get_by_uuid(state.pool.clone(), &uuid)?)
}

#[tauri::command]
pub fn rename_note(
    pool_wrapper: State<'_, PoolWrapper>,
    writer_wrapper: State<'_, WriterWrapper>,
    uuid: NoteId,
    title: &str,
) -> Result<(), ()> {
    let result = notes::rename_note(
        pool_wrapper.pool.clone(),
        writer_wrapper.writer.clone(),
        uuid,
        title,
    );
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[tauri::command]
pub fn create_note(state: State<'_, PoolWrapper>, title: &str) -> Result<String, ()> {
    let new_note = notes::create_note(state.pool.clone(), title);
    println!("Creating note {title}");
    match new_note {
        Ok(note) => {
            let json = serde_json::to_string(&note);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}
