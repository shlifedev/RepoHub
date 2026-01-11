    use std::process::Output;

    pub struct Git;

    impl Git {
        /// git command 실행
        pub async fn run_command(work_dir: &str, command: &str) -> (bool, Output) {
            println!("{work_dir}에서 {command} 명령어를 실행합니다.");

            let program = "git";
            let args = command.split_whitespace().collect::<Vec<&str>>();

            let output = tokio::process::Command::new(program)
                .args(&args)
                .current_dir(work_dir)
                .output()
                .await
                .unwrap();

            let has_error = !output.status.success();

            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                println!("명령어 실행 결과: {}", result);
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("명령어 실행 오류: {}", error);
            }

            (has_error, output)
        }

        // remotr branches 가져오기.
        pub async fn remote_branch_list(work_dir: &str) -> Option<Vec<String>> {
            let (has_error, output) = Self::run_command(work_dir, "branch -r").await;

            if !has_error {
                let result = String::from_utf8_lossy(&output.stdout);
                let branches = result
                    .lines()
                    .map(|line| line.trim().to_string())
                    .collect::<Vec<String>>();
                Some(branches)
            } else {
                None
            }
        }

        pub async fn remote_url(work_dir: &str) -> Option<String> {
            let (has_error, output) = Self::run_command(work_dir, "config --get remote.origin.url").await;

            if !has_error {
                let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Some(url)
            } else {
                None
            }
        }

        pub async fn is_git_directory(work_dir: &str) -> bool {
            let (has_error, _) = Self::run_command(work_dir, "rev-parse --is-inside-work-tree").await;
            !has_error
        }

        // git 가 설치되어 있는지 확인
        pub async fn is_git_in_path() -> bool {
            let (has_error, _) = Self::run_command(".", "--version").await;
            !has_error
        }

        /// 모든 변경사항을 버립니다.
        pub async fn reset_hard(work_dir: &str) -> bool {
            println!("{work_dir}의 변경사항을 모두 버립니다.");
            let (has_error, _) = Self::run_command(work_dir, "reset --hard").await;
            !has_error
        }

        /// 특정 브랜치를 체크아웃합니다.
        pub async fn check_out(work_dir: &str, branch: &str, discard_all: bool) -> bool {
            println!("{work_dir}의 {branch}를 체크아웃합니다.");
            if discard_all {
                Self::reset_hard(work_dir).await;
            }
            let (has_error, _) = Self::run_command(work_dir, &format!("checkout {}", branch)).await;
            !has_error
        }

        /// 원격 저장소의 변경사항을 가져옵니다.
        pub async fn fetch(work_dir: &str) -> bool {
            println!("{work_dir}에서 원격 저장소의 변경사항을 가져옵니다.");
            let (has_error, _) = Self::run_command(work_dir, "fetch").await;
            !has_error
        }
    }