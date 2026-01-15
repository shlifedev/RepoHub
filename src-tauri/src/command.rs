use std::sync::Mutex;
use std::path::Path;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;
use crate::AppState;
use crate::modules::types::{RepositoryInfo, CloneProgressEvent, CloneCompleteEvent, TagInfo};
use crate::modules::git::Git;
use tauri_specta::Event;

#[tauri::command]
#[specta::specta]
pub fn get_root_path(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.path_root.clone()
}

#[tauri::command]
#[specta::specta]
pub fn set_root_path(app: AppHandle, state: State<'_, Mutex<AppState>>, path: String) -> String {
    {
        let mut st = state.lock().unwrap();
        st.path_root = path.clone();
    }
    save_state(app, state).ok();
    path
}

#[tauri::command]
#[specta::specta]
pub fn validate_repo_name(name: String) -> Result<bool, String> {
    if name.is_empty() {
        return Err("Repository name cannot be empty".to_string());
    }
    let valid = name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !valid {
        return Err("Repository name can only contain letters, numbers, underscores, and dashes".to_string());
    }
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub fn get_repositories(state: State<'_, Mutex<AppState>>) -> Vec<RepositoryInfo> {
    let state = state.lock().unwrap();
    state.local_repositories.clone()
}

#[tauri::command]
#[specta::specta]
pub async fn clone_repository(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    remote_url: String,
    project_name: String,
) -> Result<RepositoryInfo, String> {
    let root_path = {
        let state = state.lock().unwrap();
        state.path_root.clone()
    };

    if root_path.is_empty() {
        return Err("Root path is not set. Please set it in Settings.".to_string());
    }

    let valid = project_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !valid || project_name.is_empty() {
        return Err("Invalid repository name".to_string());
    }

    let final_path = format!("{}/{}", root_path, project_name);
    if Path::new(&final_path).exists() {
        return Err(format!("Directory '{}' already exists", project_name));
    }

    let temp_path = format!("{}/.tmp_{}", root_path, project_name);

    println!("[DEBUG] Starting clone: {} -> {}", remote_url, temp_path);

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 0,
        message: "Starting...".to_string(),
    }.emit(&app).ok();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("[DEBUG] Emitting 10% progress");

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 10,
        message: "Cloning repository...".to_string(),
    }.emit(&app).ok();

    println!("[DEBUG] Calling Git::clone");
    let clone_result = Git::clone(&temp_path, &remote_url).await;
    println!("[DEBUG] Git::clone returned: {}", clone_result);

    if !clone_result {
        if Path::new(&temp_path).exists() {
            std::fs::remove_dir_all(&temp_path).ok();
        }
        CloneCompleteEvent {
            repo_name: project_name.clone(),
            success: false,
            error_message: Some("Failed to clone repository".to_string()),
        }.emit(&app).ok();
        return Err("Failed to clone repository".to_string());
    }

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 60,
        message: "Fetching tags...".to_string(),
    }.emit(&app).ok();

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 80,
        message: "Moving to final location...".to_string(),
    }.emit(&app).ok();

    if let Err(e) = std::fs::rename(&temp_path, &final_path) {
        std::fs::remove_dir_all(&temp_path).ok();
        CloneCompleteEvent {
            repo_name: project_name.clone(),
            success: false,
            error_message: Some(format!("Failed to move repository: {}", e)),
        }.emit(&app).ok();
        return Err(format!("Failed to move repository: {}", e));
    }

    let branch = Git::current_branch(&final_path).await.unwrap_or("main".to_string());

    Git::fetch_tags(&final_path).await;
    let tags = Git::get_filtered_tags(&final_path, 10).await.unwrap_or_default();
    let display_names: Vec<String> = tags.iter().map(|(_, d)| d.clone()).collect();
    let original_tags: Vec<String> = tags.iter().map(|(o, _)| o.clone()).collect();
    let current_version = display_names.first().cloned().unwrap_or_default();

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 90,
        message: "Saving repository info...".to_string(),
    }.emit(&app).ok();

    let new_repo = {
        let mut state = state.lock().unwrap();
        let id_max = state.local_repositories.iter().map(|repo| repo.id).max().unwrap_or(0);
        
        let repo = RepositoryInfo {
            id: id_max + 1,
            name: project_name.clone(),
            remote_url,
            branch,
            path: final_path,
            game_version: current_version,
            game_versions: display_names,
            server: "".to_string(),
            server_options: original_tags,
            has_warning: false,
            last_sync_time: Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        };
        
        state.local_repositories.push(repo.clone());
        repo
    };

    save_state(app.clone(), state).ok();

    CloneProgressEvent {
        repo_name: project_name.clone(),
        progress: 100,
        message: "Clone complete!".to_string(),
    }.emit(&app).ok();

    CloneCompleteEvent {
        repo_name: project_name,
        success: true,
        error_message: None,
    }.emit(&app).ok();

    Ok(new_repo)
}

#[tauri::command]
#[specta::specta]
pub fn add_project(state: State<'_, Mutex<AppState>>, remote_url: String, project_name: String) -> Vec<RepositoryInfo> {
    let mut state = state.lock().unwrap();

    let id_max = state.local_repositories.iter().map(|repo| repo.id).max().unwrap_or(0);
    let already_exists = state.local_repositories.iter().any(|repo| repo.remote_url == remote_url);
    if already_exists {
        return state.local_repositories.clone();
    }
    if project_name.trim().is_empty() {
        return state.local_repositories.clone();
    }

    state.local_repositories.push(RepositoryInfo {
        id: id_max + 1,
        name: project_name,
        remote_url,
        branch: "main".to_string(),
        game_version: "".to_string(),
        game_versions: vec![],
        path: "".to_string(),
        server: "".to_string(),
        server_options: vec![],
        has_warning: false,
        last_sync_time: None,
    });

    state.local_repositories.clone()
}

#[tauri::command]
#[specta::specta]
pub async fn get_filtered_tags(repo_path: String) -> Result<Vec<TagInfo>, String> {
    if !Path::new(&repo_path).exists() {
        return Err("Repository path does not exist".to_string());
    }

    match Git::get_filtered_tags(&repo_path, 10).await {
        Some(tags) => {
            let tag_infos: Vec<TagInfo> = tags
                .into_iter()
                .map(|(original, display)| TagInfo {
                    original_tag: original,
                    display_name: display,
                })
                .collect();
            Ok(tag_infos)
        }
        None => Err("Failed to get tags".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_repository(
    state: State<'_, Mutex<AppState>>,
    repo_id: u32,
) -> Result<RepositoryInfo, String> {
    let repo_path = {
        let state = state.lock().unwrap();
        state.local_repositories
            .iter()
            .find(|r| r.id == repo_id)
            .map(|r| r.path.clone())
    };

    let repo_path = repo_path.ok_or("Repository not found")?;

    if !Path::new(&repo_path).exists() {
        return Err("Repository path does not exist".to_string());
    }

    Git::fetch_tags(&repo_path).await;

    let tags = Git::get_filtered_tags(&repo_path, 10).await.unwrap_or_default();
    let display_names: Vec<String> = tags.iter().map(|(_, d)| d.clone()).collect();
    let original_tags: Vec<String> = tags.iter().map(|(o, _)| o.clone()).collect();
    
    let current_version = display_names.first().cloned().unwrap_or_default();
    let sync_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut state = state.lock().unwrap();
    if let Some(repo) = state.local_repositories.iter_mut().find(|r| r.id == repo_id) {
        repo.game_versions = display_names;
        repo.server_options = original_tags;
        repo.game_version = current_version;
        repo.last_sync_time = Some(sync_time);
        return Ok(repo.clone());
    }

    Err("Repository not found".to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_repository(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    repo_id: u32,
) -> Result<bool, String> {
    let repo_path = {
        let state = state.lock().unwrap();
        state.local_repositories
            .iter()
            .find(|r| r.id == repo_id)
            .map(|r| r.path.clone())
    };

    let repo_path = repo_path.ok_or("Repository not found")?;

    if Path::new(&repo_path).exists() {
        std::fs::remove_dir_all(&repo_path)
            .map_err(|e| format!("Failed to delete repository: {}", e))?;
    }

    {
        let mut st = state.lock().unwrap();
        st.local_repositories.retain(|r| r.id != repo_id);
    }
    
    save_state(app, state).ok();

    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub fn save_state(app: AppHandle, state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let state = state.lock().unwrap();
    
    let store = app.store("db.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;
    
    store.set("path_root", serde_json::json!(&state.path_root));
    store.set("local_repositories", serde_json::json!(&state.local_repositories));
    
    store.save()
        .map_err(|e| format!("Failed to save store: {}", e))?;
    
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub fn load_state(app: AppHandle, state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let store = app.store("db.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;
    
    let mut state = state.lock().unwrap();
    
    if let Some(path_root) = store.get("path_root") {
        if let Some(path) = path_root.as_str() {
            state.path_root = path.to_string();
        }
    }
    
    if let Some(repos) = store.get("local_repositories") {
        if let Ok(repositories) = serde_json::from_value::<Vec<RepositoryInfo>>(repos.clone()) {
            state.local_repositories = repositories;
        }
    }
    
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub async fn change_version(
    state: State<'_, Mutex<AppState>>,
    repo_id: u32,
    tag: String,
) -> Result<RepositoryInfo, String> {
    let repo_path = {
        let state = state.lock().unwrap();
        state.local_repositories
            .iter()
            .find(|r| r.id == repo_id)
            .map(|r| r.path.clone())
    };

    let repo_path = repo_path.ok_or("Repository not found")?;

    if !Path::new(&repo_path).exists() {
        return Err("Repository path does not exist".to_string());
    }

    let success = Git::checkout_tag(&repo_path, &tag, true).await;
    if !success {
        return Err("Failed to checkout to tag".to_string());
    }

    let mut state = state.lock().unwrap();
    if let Some(repo) = state.local_repositories.iter_mut().find(|r| r.id == repo_id) {
        if let Some(idx) = repo.server_options.iter().position(|t| t == &tag) {
            if let Some(display) = repo.game_versions.get(idx) {
                repo.game_version = display.clone();
            }
        }
        return Ok(repo.clone());
    }

    Err("Repository not found".to_string())
}
