// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



use tauri_plugin_sql::{Migration, MigrationKind, SqliteConfig};

fn main() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:mydatabase.db", migrations)
                .add_sqlite_options("sqlite:mydatabase.db", SqliteConfig{key:"my_database_key", journal_mode:"OFF", foreign_keys:true, read_only:false,..Default::default()})
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

