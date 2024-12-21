// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//mod commands;
mod database;
//mod lexical;
mod macros;
//mod models;
//mod schema;
//mod search;
//mod setup;
//mod utils;
//use specta_typescript::Typescript;
//use tauri_specta::{collect_commands, Builder};

fn main() {
    //let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
    //commands::notes::get_note,
    //commands::notes::get_all_notes,
    //commands::notes::update_note,
    //commands::notes::get_notes_by_title,
    //commands::notes::get_notes_by_content,
    //commands::notes::create_note,
    //commands::notes::rename_note,
    //commands::notes::get_direct_tags,
    //commands::notes::add_new_tag,
    //commands::notes::add_tag,
    //commands::settings::get_setting,
    //commands::settings::get_setting_or_set,
    //commands::settings::update_setting,
    //commands::tags::get_available_tags_containing,
    //]);

    //#[cfg(debug_assertions)] // <- Only export on non-release builds
    //builder
    //    .export(
    //        Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
    //        "../src/bindings.ts",
    //    )
    //    .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
