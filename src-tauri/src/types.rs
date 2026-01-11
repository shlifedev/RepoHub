use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Clone)]
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


#[derive(Debug, Clone, Serialize, Deserialize, Type, Event)]
pub struct MyEvent(String);
