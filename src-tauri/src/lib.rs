use std::sync::Mutex;
use specta_typescript::Typescript;
use tauri::State;
use tauri_specta::{collect_commands, collect_events};
use modules::types::AppInitializeEvent;
use crate::modules::types::RepositoryInfo;

pub mod modules {
    pub mod git;
    pub mod types;
    pub mod appDb;
}
pub mod commands{

}


struct AppState {
    count : u32,
    path_root : String,
    registered_repo_urls : Vec<String>,
    local_repositories : Vec<RepositoryInfo>
}

#[tauri::command]
#[specta::specta]
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32{
    println!("increase_counter");
    let mut state = state.lock().unwrap();
    state.count += 1;
    state.count
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri_specta::Builder::<tauri::Wry>::new()
        // 사용 할 커맨드 등록
        .commands(collect_commands![increase_counter]).
        // 사용할 이벤트 등록
        events(collect_events![AppInitializeEvent]);


    // Typescript Binding 내보내기 [이벤트, 커멘드]
    builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");


    // 앱 빌드.
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // ?
        .manage(Mutex::new(AppState {
            count: 0,
            path_root: "".to_string(),
            registered_repo_urls: vec![],
            local_repositories: vec![] })) // 상태 관리할 객체.
        .invoke_handler(builder.invoke_handler()) // 핸들러 등록
        .run(tauri::generate_context!()) // 실행
        .expect("error while running tauri application");
}
