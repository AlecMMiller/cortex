use crate::models::NoteId;
use crate::notes;
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
pub fn get_all_notes<'a>(state: State<'_, PoolWrapper>) -> Result<String, ()> {
    let all_notes = notes::get_all(state.pool.clone());

    match all_notes {
        Ok(all_notes) => {
            let json = serde_json::to_string(&all_notes);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[tauri::command]
pub fn get_notes_by_title<'a>(state: State<'_, PoolWrapper>, title: &str) -> Result<String, ()> {
    let all_notes = notes::get_all(state.pool.clone());
    println!("{title}");

    match all_notes {
        Ok(all_notes) => {
            let json = serde_json::to_string(&all_notes);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[tauri::command]
pub fn get_note<'a>(state: State<'_, PoolWrapper>, uuid: NoteId) -> Result<String, ()> {
    let note = notes::get_by_uuid(state.pool.clone(), uuid);

    match note {
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
