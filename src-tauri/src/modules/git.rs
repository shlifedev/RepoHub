use std::process::Output;

pub struct Git;

impl Git {
    pub async fn run_command(work_dir: &str, command: &str) -> (bool, Output) {
        let program = "git";
        let args = command.split_whitespace().collect::<Vec<&str>>();

        let output = tokio::process::Command::new(program)
            .args(&args)
            .current_dir(work_dir)
            .output()
            .await
            .unwrap();

        let has_error = !output.status.success();
        (has_error, output)
    }

    pub async fn clone(target_dir: &str, remote_url: &str) -> bool {
        let target = target_dir.to_string();
        let url = remote_url.to_string();
        
        println!("[Git::clone] Starting clone: {} -> {}", url, target);
        
        let result = tokio::task::spawn_blocking(move || {
            println!("[Git::clone] spawn_blocking started");
            let output = std::process::Command::new("git")
                .args(["clone", &url, &target])
                .output();
            println!("[Git::clone] git command finished");
            output
        }).await;

        match result {
            Ok(Ok(output)) => {
                println!("[Git::clone] Success: {}", output.status.success());
                if !output.status.success() {
                    println!("[Git::clone] stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
                output.status.success()
            },
            Ok(Err(e)) => {
                println!("[Git::clone] Command error: {}", e);
                false
            },
            Err(e) => {
                println!("[Git::clone] Task error: {}", e);
                false
            },
        }
    }

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

    pub async fn current_branch(work_dir: &str) -> Option<String> {
        let (has_error, output) = Self::run_command(work_dir, "rev-parse --abbrev-ref HEAD").await;

        if !has_error {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Some(branch)
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

    pub async fn is_git_in_path() -> bool {
        let (has_error, _) = Self::run_command(".", "--version").await;
        !has_error
    }

    pub async fn reset_hard(work_dir: &str) -> bool {
        let (has_error, _) = Self::run_command(work_dir, "reset --hard").await;
        !has_error
    }

    pub async fn check_out(work_dir: &str, branch: &str, discard_all: bool) -> bool {
        if discard_all {
            Self::reset_hard(work_dir).await;
        }
        let (has_error, _) = Self::run_command(work_dir, &format!("checkout {}", branch)).await;
        !has_error
    }

    pub async fn fetch(work_dir: &str) -> bool {
        let (has_error, _) = Self::run_command(work_dir, "fetch").await;
        !has_error
    }

    pub async fn fetch_tags(work_dir: &str) -> bool {
        let (has_error, _) = Self::run_command(work_dir, "fetch --tags").await;
        !has_error
    }

    pub async fn get_tags(work_dir: &str, limit: usize) -> Option<Vec<String>> {
        let (has_error, output) = Self::run_command(work_dir, "tag --sort=-creatordate").await;

        if !has_error {
            let result = String::from_utf8_lossy(&output.stdout);
            let tags: Vec<String> = result
                .lines()
                .take(limit)
                .map(|line| line.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect();
            Some(tags)
        } else {
            None
        }
    }

    pub async fn get_filtered_tags(work_dir: &str, limit: usize) -> Option<Vec<(String, String)>> {
        let (has_error, output) = Self::run_command(work_dir, "tag --sort=-creatordate").await;

        if !has_error {
            let result = String::from_utf8_lossy(&output.stdout);
            let tags: Vec<(String, String)> = result
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|t| !t.is_empty())
                .filter(|tag| {
                    let lower = tag.to_lowercase();
                    lower.contains("dev") || lower.contains("qa")
                })
                .take(limit)
                .map(|tag| {
                    let display = Self::format_tag_display(&tag);
                    (tag, display)
                })
                .collect();
            Some(tags)
        } else {
            None
        }
    }

    fn format_tag_display(tag: &str) -> String {
        let lower = tag.to_lowercase();
        
        let prefix = if lower.contains("dev") {
            "dev"
        } else if lower.contains("qa") {
            "qa"
        } else {
            return tag.to_string();
        };

        match Self::extract_version(tag) {
            Some(v) => format!("{}-{}", prefix, v),
            None => tag.to_string(),
        }
    }

    fn extract_version(tag: &str) -> Option<String> {
        let mut version_parts: Vec<char> = Vec::new();
        let mut in_version = false;
        let mut dot_count = 0;

        for ch in tag.chars() {
            if ch.is_ascii_digit() {
                in_version = true;
                version_parts.push(ch);
            } else if ch == '.' && in_version && dot_count < 2 {
                version_parts.push(ch);
                dot_count += 1;
            } else if in_version {
                break;
            }
        }

        if version_parts.is_empty() {
            None
        } else {
            let version: String = version_parts.into_iter().collect();
            let version = version.trim_end_matches('.').to_string();
            if version.is_empty() { None } else { Some(version) }
        }
    }

    pub async fn checkout_tag(work_dir: &str, tag: &str, discard_all: bool) -> bool {
        if discard_all {
            Self::reset_hard(work_dir).await;
        }
        let (has_error, _) = Self::run_command(work_dir, &format!("checkout tags/{}", tag)).await;
        !has_error
    }
}