use crate::commands::Error;
use crate::models::{Note, NoteId};
use crate::notes;
use crate::notes::NoteTitle;
use crate::PoolWrapper;
use tauri::State;

#[tauri::command]
pub fn update_note(state: State<'_, PoolWrapper>, uuid: NoteId, body: &str) -> Result<(), ()> {
    let result = notes::update_body(state.pool.clone(), uuid, body);
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
    Ok(notes::get_all(state.pool.clone())?)
}

#[tauri::command]
pub fn get_notes_by_title<'a>(
    state: State<'_, PoolWrapper>,
    _title: &str,
) -> Result<Vec<NoteTitle>, Error> {
    Ok(notes::get_all(state.pool.clone())?)
}

#[tauri::command]
pub fn get_note<'a>(state: State<'_, PoolWrapper>, uuid: NoteId) -> Result<Note, Error> {
    Ok(notes::get_by_uuid(state.pool.clone(), uuid)?)
}

#[tauri::command]
pub fn rename_note(state: State<'_, PoolWrapper>, uuid: NoteId, title: &str) -> Result<(), ()> {
    let result = notes::rename_note(state.pool.clone(), uuid, title);
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
