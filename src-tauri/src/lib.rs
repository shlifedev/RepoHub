mod command;

use std::sync::Mutex;
use specta_typescript::Typescript;
use tauri::{Manager, State};
use tauri_specta::{collect_commands, collect_events};
use modules::types::{AppInitializeEvent, CloneProgressEvent, CloneCompleteEvent};
use crate::modules::types::RepositoryInfo;
use crate::command::{get_root_path, set_root_path, add_project, clone_repository, validate_repo_name, get_repositories, get_filtered_tags, refresh_repository, change_version, delete_repository, save_state, load_state};

pub mod modules {
    pub mod git;
    pub mod types;
}

pub struct AppState {
    count: u32,
    pub path_root: String,
    pub local_repositories: Vec<RepositoryInfo>
}

#[tauri::command]
#[specta::specta]
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.count += 1;
    state.count
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            increase_counter,
            get_root_path,
            set_root_path,
            add_project,
            clone_repository,
            validate_repo_name,
            get_repositories,
            get_filtered_tags,
            refresh_repository,
            change_version,
            delete_repository,
            save_state,
            load_state
        ])
        .events(collect_events![AppInitializeEvent, CloneProgressEvent, CloneCompleteEvent]);

    builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    let invoke_handler = builder.invoke_handler();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Mutex::new(AppState {
            count: 0,
            path_root: "".to_string(),
            local_repositories: vec![]
        }))
        .setup(move |app| {
            builder.mount_events(app);
            
            let handle = app.handle().clone();
            let state = app.state::<Mutex<AppState>>();
            load_state(handle, state).ok();
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
