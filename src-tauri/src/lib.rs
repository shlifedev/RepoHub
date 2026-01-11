mod types;
pub mod git;

use std::fs;
use std::path::PathBuf;
use specta_typescript::Typescript;
use tauri::{AppHandle, Builder, Manager};
use types::RepositoryInfo;
use tauri_specta::{collect_commands, collect_events};
use crate::types::MyEvent;

fn get_data_path(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join("userdata.json")
}

#[tauri::command]
#[specta::specta]
fn save_repositories(repos: Vec<RepositoryInfo>, app: AppHandle) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().unwrap();

    // 디렉토리가 없으면 생성
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    }

    let path = get_data_path(&app);
    let json = serde_json::to_string_pretty(&repos).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    println!("Saved {} repositories", repos.len());
    Ok(())
}

#[tauri::command]
#[specta::specta]
fn load_repositories(app: AppHandle) -> Result<Vec<RepositoryInfo>, String> {
    let path = get_data_path(&app);

    if path.exists() {
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let repos: Vec<RepositoryInfo> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        println!("Loaded {} repositories", repos.len());
        Ok(repos)
    } else {
        println!("No saved repositories found");
        Ok(Vec::new())
    }
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn hello_world(my_name: String)   {
    println!("Hello {}!", my_name);
}

// 기존 greet 함수 유지
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {



    let mut builder = tauri_specta::Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![hello_world, ]).
        events(collect_events![MyEvent]);


    builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
