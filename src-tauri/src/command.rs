use std::sync::Mutex;
use tauri::State;
use crate::AppState;
use crate::modules::types::RepositoryInfo;

#[tauri::command]
#[specta::specta]
fn add_project(state: State<'_, Mutex<AppState>>, remote_url:String, project_name : String) -> Vec<RepositoryInfo>
{
    let mut state = state.lock().unwrap();

    /*
        path_root 를 기준으로
        1. project_name 폴더가 존재하는지 확인. 없으면 생성.(있으면 오류)
        2. git clone remote_url project_name 실행.
        3. local_repositories 에 필요한 정보 추가.
    */

    let idMax = state.local_repositories.iter().map(|repo| repo.id).max().unwrap_or(0);
    let alreadyExists = state.local_repositories.iter().any(|repo| repo.remote_url == remote_url);
    if alreadyExists {
        return state.local_repositories.clone();
    }
    if project_name.trim().is_empty() {
        return state.local_repositories.clone();
    }

    state.local_repositories.push(RepositoryInfo {
        id: idMax + 1,
        name: project_name,
        remote_url,
        branch: "main".to_string(),
        game_version: "".to_string(),
        game_versions: vec![],
        path: "".to_string(),
        server: "".to_string(),
        server_options: vec![],
        has_warning: false,
    });

    return state.local_repositories.clone();
}
