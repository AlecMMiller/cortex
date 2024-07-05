// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::schema::initialize_schema;
use lexical::EditorState;
use serde_json::Error;
use sqlx::SqlitePool;

mod lexical;
mod database;
mod types;

fn deserialize_editor(state: &str) -> Result<EditorState, Error> {
    let res: EditorState = EditorState::from_str(state)?;

    //println!("{res:?}");

    Ok(res)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn editor_change_state(state: &str) {
    let state = deserialize_editor(state);
    match state {
        Ok(_result) => return,
        Err(error) => println!("{error}"),
    }
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    initialize_schema(&pool).await.expect("Failed to initialize schema");

    //initialize_schema(&pool).await.expect("Failed to initialize schema");

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![editor_change_state])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
