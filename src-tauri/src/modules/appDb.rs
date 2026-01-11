use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Mutex;
use crate::modules::types::RepositoryInfo;

#[derive(Serialize, Deserialize, Type, Clone, Default)]
pub struct AppDb {
    repository_infos: Vec<RepositoryInfo>,
    pub auth_token: String,
    pub root_path: String,
    pub app_version: String,
}
 
pub struct AppDbState(pub Mutex<AppDb>);

impl AppDb {
    const DB_FILE_NAME: &'static str = "appDb.json";

    pub fn load_from_json_file() -> Self {
        std::fs::read_to_string(Self::DB_FILE_NAME)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save_to_json_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(Self::DB_FILE_NAME, json)?;
        Ok(())
    }

    pub fn add_repository(&mut self, repo_url: String, create_folder_name: String) -> Result<(), String> { 
        Ok(())
    }

    pub fn remove_repository(&mut self, work_dir: String) -> Result<(), String> { 
        Ok(())
    }
}