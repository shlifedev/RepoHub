use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct RepositoryInfo {
    pub id: u32,
    pub name: String,
    pub remote_url: String,
    pub branch: String,
    pub path: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    pub server: String,
    #[serde(rename = "serverOptions")]
    pub server_options: Vec<String>,
    #[serde(rename = "hasWarning")]
    pub has_warning: bool,
    #[serde(rename = "lastSyncTime")]
    pub last_sync_time: Option<String>,
}

#[derive(Clone, Type, Event)]
pub struct AppInitializeEvent {
    pub repository_datas: Vec<RepositoryInfo>,
    pub auth_token: String,
    pub root_path: String,
    pub app_version: String,
}

#[derive(Clone, Serialize, Deserialize, Type, Event)]
pub struct CloneProgressEvent {
    pub repo_name: String,
    pub progress: u32,
    pub message: String,
    #[serde(rename = "receivedBytes")]
    pub received_bytes: Option<f64>,
    #[serde(rename = "totalObjects")]
    pub total_objects: Option<u32>,
    #[serde(rename = "receivedObjects")]
    pub received_objects: Option<u32>,
    pub speed: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Type, Event)]
pub struct CloneCompleteEvent {
    pub repo_name: String,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct TagInfo {
    #[serde(rename = "originalTag")]
    pub original_tag: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}