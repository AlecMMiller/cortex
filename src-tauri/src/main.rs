// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use models::NoteId;
use std::fs::create_dir_all;
use tauri::{Manager, State};
mod models;
mod schema;

mod lexical;
mod macros;
mod notes;
mod utils;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// fn deserialize_editor(state: &str) -> Result<EditorState, Error> {
//     let res: EditorState = EditorState::from_str(state)?;

//     //println!("{res:?}");

//     Ok(res)
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn editor_change_state(state: State<'_, PoolWrapper>, uuid: NoteId, body: &str) -> Result<(), ()> {
    let result = notes::update_body(state.pool.clone(), uuid, body);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

struct PoolWrapper {
    pub pool: SqlitePool,
}

#[tauri::command]
fn get_last_updated<'a>(state: State<'_, PoolWrapper>) -> Result<String, ()> {
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
fn rename_note(state: State<'_, PoolWrapper>, uuid: NoteId, title: &str) -> Result<(), ()> {
    let result = notes::rename_note(state.pool.clone(), uuid, title);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .expect("This should never be None");

            create_dir_all(&path).unwrap();
            let db_path = path.join("data.db");
            let db_path = db_path.to_str().expect("This should never be None");

            let mut conn = SqliteConnection::establish(db_path)
                .expect("Could not establish connection to database");
            conn.run_pending_migrations(MIGRATIONS)
                .expect("Could not run migrations");

            let manager = ConnectionManager::<SqliteConnection>::new(db_path);
            let pool = Pool::builder()
                .build(manager)
                .expect("Could not create connection pool");
            let pool = PoolWrapper { pool };

            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            editor_change_state,
            get_last_updated,
            rename_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
