// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;

use tauri_plugin_fs::FsExt;
use lexical::EditorState;
use serde_json::Error;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{Manager, State};

mod lexical;
mod notes;

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

#[tauri::command]
async fn get_last_updated<'a>(pool: State<'_, SqlitePool>) -> Result<String, ()> {
    let last_updated = notes::get_last_updated(&pool).await;
    match last_updated {
        Some(note) => {
            let json = serde_json::to_string(&note);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(())
            }
        }
        None => Err(())
    }
}

#[tauri::command]
async fn create_note<'a>(name: &str, pool: State<'_, SqlitePool>) -> Result<String, String> {
    println!("Creating note with name: {}", name);
    let note_id = notes::create(&pool, name).await;
    match note_id {
        Ok(note_id) => Ok(note_id.into()),
        Err(_) => Err("Failed to create note".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let path = app.path().app_data_dir().expect("This should never be None");
  
            create_dir_all(&path).unwrap();
            let db_path = path.join("data.db");
            let runtime = tokio::runtime::Runtime::new().expect("Tokio should be able to create a runtime");
            let pool = runtime.block_on(async {
                let conn = SqliteConnectOptions::new().filename(db_path).create_if_missing(true);
                let pool = SqlitePool::connect_with(conn).await.expect("SQLX should be able to connect to the database");
                sqlx::migrate!().run(&pool).await.expect("SQLX should be able to run migrations");
                pool
            });

            app.manage(pool);
            Ok(())
         })
        .invoke_handler(tauri::generate_handler![create_note, editor_change_state, get_last_updated])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
