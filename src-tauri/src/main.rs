#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn test(map: HashMap<i32, i32>) {
    println!("{map:#?}");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
