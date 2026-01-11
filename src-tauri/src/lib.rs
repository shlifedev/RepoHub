use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events};
use modules::types::AppInitializeEvent;


pub mod modules {
    pub mod git;
    pub mod types;
    pub mod appDb;
}

pub mod commands{
    pub mod sample;
}

#[tauri::command]
#[specta::specta]
fn refresh_command() {

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri_specta::Builder::<tauri::Wry>::new()
        // 사용 할 커맨드 등록
        .commands(collect_commands![refresh_command]).
        // 사용할 이벤트 등록
        events(collect_events![AppInitializeEvent]);


    // Typescript Binding 내보내기 [이벤트, 커멘드]
    builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");


    // 앱 빌드.
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
