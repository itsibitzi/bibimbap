use db::Database;
use tauri::State;

mod db;
mod model;

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn add_deck(db: State<'_, Database>, name: String) -> Result<(), String> {
    db.insert_deck(&name).await;
    Ok(())
}

fn main() {
    let db = Database::new().await?;
    db.migrate().await?;

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(db)
        .manage(key_store)
        .invoke_handler(tauri::generate_handler![add_deck])
        .on_window_event(move |event| {
            match event.event() {
                WindowEvent::CloseRequested { .. } => {
                    let db = event.window().state::<Database>();
                    println!("Closing database");
                    let closed = block_in_place(|| block_on(db.close()));

                    if let Err(e) = closed {
                        eprintln!("Failed to close database: {}", e);
                    }
                }
                _ => (),
            };
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
