## Workflow

프로젝트 구조를 다음처럼 가져가려고 함.

### Structure 

```
# 프런트엔드

/src - frontend # 뷰 코드
ㄴ /binding.ts # specta를 사용해 자동 생성된 typescript binder. 여기에는 tauri backend와 통신할때 사용할 수 있는 이벤트, 커맨드 등 있음. 

# 백엔드

/src-tauri - rust app backend
ㄴ /commands # 모든 커맨드를 이 곳에 작성.
ㄴ /events # 모든 이벤트도 이 곳에 작성.
ㄴ /modules # 러스트 측에서 공유해서 사용할 기타 모듈들.
    ㄴ /types.rs # Shared Types.
    
lib.rs/main.rs - 러스트 main 시작점 및 라이브러리 정의
```

### State Management

단순한 어플리케이션 이므로 `AppState` 하나로 관리. (상태 관리할 객체가 생기면 등록이 필요)

```rust
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState {
            count: 0,
            path_root: "".to_string(),
            registered_repo_urls: vec![],
            local_repositories: vec![] })) // 상태 관리할 객체.
```


- 상태를 등록해두면 `command` 측에서 (아래예시) 자동으로 상태값을 주입받을 수 있다.  `state: State<'_, Mutex<AppState>>` 가 그 예시. 
### 커맨드
프론트 측에서 백엔드 측 코드실행, 상태조작 등을 이로 실행

두 가지 어노테이션을 필수적으로 붙여서 작성
```rust
#[tauri::command] // tauri에 커맨드임을 알리는 어노테이션
#[specta::specta] // specta에 자동 생성해야하는 코드를 마킹하기 위한 어노테이션
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32{
    println!("increase_counter");
    let mut state = state.lock().unwrap();
    state.count += 1;
    state.count
}
``` 

이후 앱 매니저에 인보크 리스트에 등록해주어야 함

```
pub fn run() {
    let mut builder = tauri_specta::Builder::<tauri::Wry>::new()
        // 사용 할 커맨드 등록
        .commands(collect_commands![increase_counter]);
       // 그 외 기타 코드들..
}
```



### 이벤트

프론트는 이벤트를 등록해놓고 뷰를 조작하는데 활용할 수 있습니다.

작성 예정 

 
