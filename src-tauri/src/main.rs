// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod database;
mod macros;
mod setup;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

fn main() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::schema::create_entity_schema,
        commands::schema::get_entity_schema
    ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
