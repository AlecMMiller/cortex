// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use lexical::EditorState;
use serde_json::Error;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tauri::State;

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

#[tokio::main]
async fn main() {
    //let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_url = "sqlite:test.db";

    if !sqlx::Sqlite::database_exists(&db_url)
        .await
        .expect("Failed to check if database exists")
    {
        sqlx::Sqlite::create_database(&db_url)
            .await
            .expect("Failed to create database");
    }

    let pool = SqlitePool::connect("sqlite:test.db").await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![create_note, editor_change_state, get_last_updated])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
