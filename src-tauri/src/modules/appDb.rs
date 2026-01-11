use std::path::Path;
use serde::{Deserialize, Serialize};
use specta::Type;
use crate::modules::types::RepositoryInfo;


#[derive(Serialize, Deserialize, Type)]
pub struct AppDb {
    repository_infos: Vec<RepositoryInfo>,
    pub auth_token: String,
    pub root_path: String,
    pub app_version: String
}

impl AppDb { 
    const DB_FILE_NAME : &'static str = "appDb.json";
    pub fn load_from_json_file() -> AppDb {
        let file = std::fs::read_to_string(Self::DB_FILE_NAME).unwrap();
        serde_json::from_str(&file).unwrap()
    }
    
    pub fn save_to_json_file(&self, path : &str) {
        std::fs::write(path, Self::DB_FILE_NAME).unwrap();
    }

    /*
        리포지토리를 추가합니다.
        repoUrl : git remote url
        create_folder_name : remote repo를 clone 할 대상 폴더, 이미 존재하는 폴더가 있을 경우 에러 반환.
    */
    pub fn add_repository(repoUrl : String, create_folder_name : String) {

    }

    
    pub fn remove_repository(workDir : String){}
    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}