use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;


#[derive(Serialize, Deserialize, Type, Clone)]
pub struct RepositoryInfo {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub branch: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    pub server: String,
    #[serde(rename = "serverOptions")]
    pub server_options: Vec<String>,
    #[serde(rename = "hasWarning")]
    pub has_warning: bool,
}


/*
    앱 초기화 이벤트.
*/
#[derive(Clone, Type, Event)]
pub struct AppInitializeEvent {
    pub repository_datas: Vec<RepositoryInfo>,
    pub auth_token: String,
    pub root_path: String,
    pub app_version: String
}