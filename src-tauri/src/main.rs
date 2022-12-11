use db::Database;
use tauri::State;

mod db;
mod hangul;
mod model;

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
async fn add_deck(db: State<'_, Database>, name: String) -> Result<(), &'static str> {
    db.insert_deck(&name).await.map_err(|_| "oh no")?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Database::new().await?;

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![add_deck])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
