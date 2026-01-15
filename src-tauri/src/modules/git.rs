use std::process::Output;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use tokio::sync::mpsc;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone, Debug)]
pub struct CloneProgress {
    pub stage: String,
    pub progress: u32,
    pub received_objects: Option<u32>,
    pub total_objects: Option<u32>,
    pub received_bytes: Option<f64>,
    pub speed: Option<String>,
}

pub struct Git;

impl Git {
    pub async fn run_command(work_dir: &str, command: &str) -> (bool, Output) {
        let program = "git";
        let args = command.split_whitespace().collect::<Vec<&str>>();

        #[cfg(windows)]
        let output = tokio::process::Command::new(program)
            .args(&args)
            .current_dir(work_dir)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .await
            .unwrap();

        #[cfg(not(windows))]
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
            
            #[cfg(windows)]
            let output = std::process::Command::new("git")
                .args(["clone", &url, &target])
                .creation_flags(CREATE_NO_WINDOW)
                .output();

            #[cfg(not(windows))]
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

    pub async fn clone_with_progress(
        target_dir: &str, 
        remote_url: &str,
        progress_tx: mpsc::Sender<CloneProgress>
    ) -> bool {
        let target = target_dir.to_string();
        let url = remote_url.to_string();
        
        println!("[Git::clone_with_progress] Starting clone: {} -> {}", url, target);
        
        let result = tokio::task::spawn_blocking(move || {
            #[cfg(windows)]
            let mut child = match std::process::Command::new("git")
                .args(["clone", "--progress", &url, &target])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .creation_flags(CREATE_NO_WINDOW)
                .spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        println!("[Git::clone_with_progress] Spawn error: {}", e);
                        return false;
                    }
                };

            #[cfg(not(windows))]
            let mut child = match std::process::Command::new("git")
                .args(["clone", "--progress", &url, &target])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        println!("[Git::clone_with_progress] Spawn error: {}", e);
                        return false;
                    }
                };

            let stderr = child.stderr.take().expect("Failed to capture stderr");
            let reader = BufReader::new(stderr);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some(progress) = Self::parse_git_progress(&line) {
                        let _ = progress_tx.blocking_send(progress);
                    }
                }
            }

            match child.wait() {
                Ok(status) => status.success(),
                Err(e) => {
                    println!("[Git::clone_with_progress] Wait error: {}", e);
                    false
                }
            }
        }).await;

        result.unwrap_or(false)
    }

    fn parse_git_progress(line: &str) -> Option<CloneProgress> {
        let line = line.trim();
        
        if line.contains("Receiving objects:") {
            return Self::parse_receiving_objects(line);
        } else if line.contains("Resolving deltas:") {
            return Self::parse_resolving_deltas(line);
        } else if line.contains("Counting objects:") || line.contains("Enumerating objects:") {
            return Some(CloneProgress {
                stage: "Counting".to_string(),
                progress: 5,
                received_objects: None,
                total_objects: None,
                received_bytes: None,
                speed: None,
            });
        } else if line.contains("Compressing objects:") {
            return Self::parse_compressing_objects(line);
        }
        
        None
    }

    fn parse_receiving_objects(line: &str) -> Option<CloneProgress> {
        let mut progress: u32 = 0;
        let mut received: Option<u32> = None;
        let mut total: Option<u32> = None;
        let mut bytes: Option<f64> = None;
        let mut speed: Option<String> = None;

        if let Some(pct_idx) = line.find('%') {
            let start = line[..pct_idx].rfind(|c: char| !c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);
            if let Ok(p) = line[start..pct_idx].trim().parse::<u32>() {
                progress = 10 + (p * 50 / 100);
            }
        }

        if let Some(paren_start) = line.find('(') {
            if let Some(paren_end) = line.find(')') {
                let inner = &line[paren_start + 1..paren_end];
                let parts: Vec<&str> = inner.split('/').collect();
                if parts.len() == 2 {
                    received = parts[0].trim().parse().ok();
                    total = parts[1].trim().parse().ok();
                }
            }
        }

        if let Some(comma_idx) = line.find("),") {
            let after_paren = &line[comma_idx + 2..];
            let size_parts: Vec<&str> = after_paren.split('|').collect();
            
            if !size_parts.is_empty() {
                let size_str = size_parts[0].trim();
                bytes = Self::parse_size_to_bytes(size_str);
            }
            
            if size_parts.len() > 1 {
                speed = Some(size_parts[1].trim().to_string());
            }
        }

        Some(CloneProgress {
            stage: "Receiving".to_string(),
            progress,
            received_objects: received,
            total_objects: total,
            received_bytes: bytes,
            speed,
        })
    }

    fn parse_resolving_deltas(line: &str) -> Option<CloneProgress> {
        let mut progress: u32 = 60;

        if let Some(pct_idx) = line.find('%') {
            let start = line[..pct_idx].rfind(|c: char| !c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);
            if let Ok(p) = line[start..pct_idx].trim().parse::<u32>() {
                progress = 60 + (p * 30 / 100);
            }
        }

        Some(CloneProgress {
            stage: "Resolving".to_string(),
            progress,
            received_objects: None,
            total_objects: None,
            received_bytes: None,
            speed: None,
        })
    }

    fn parse_compressing_objects(line: &str) -> Option<CloneProgress> {
        let mut progress: u32 = 8;

        if let Some(pct_idx) = line.find('%') {
            let start = line[..pct_idx].rfind(|c: char| !c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);
            if let Ok(p) = line[start..pct_idx].trim().parse::<u32>() {
                progress = 5 + (p * 5 / 100);
            }
        }

        Some(CloneProgress {
            stage: "Compressing".to_string(),
            progress,
            received_objects: None,
            total_objects: None,
            received_bytes: None,
            speed: None,
        })
    }

    fn parse_size_to_bytes(size_str: &str) -> Option<f64> {
        let size_str = size_str.trim();
        let parts: Vec<&str> = size_str.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let num: f64 = parts[0].parse().ok()?;
        let unit = parts.get(1).unwrap_or(&"B");

        let multiplier: u64 = match unit.to_uppercase().as_str() {
            "B" | "BYTES" => 1,
            "KIB" | "KB" => 1024,
            "MIB" | "MB" => 1024 * 1024,
            "GIB" | "GB" => 1024 * 1024 * 1024,
            _ => 1,
        };

        Some(num * multiplier as f64)
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

    /// Check if a specific remote branch exists (e.g., "dev", "qa")
    pub async fn has_remote_branch(work_dir: &str, branch_name: &str) -> bool {
        if let Some(branches) = Self::remote_branch_list(work_dir).await {
            branches.iter().any(|b| {
                let normalized = b.replace("origin/", "");
                normalized.eq_ignore_ascii_case(branch_name)
            })
        } else {
            false
        }
    }

    /// Checkout to a remote branch with fetch and pull
    pub async fn checkout_remote_branch(work_dir: &str, branch: &str, discard_all: bool) -> bool {
        if discard_all {
            Self::reset_hard(work_dir).await;
        }
        
        Self::fetch(work_dir).await;
        
        let (has_error, _) = Self::run_command(work_dir, &format!("checkout {}", branch)).await;
        
        if has_error {
            let (has_error2, _) = Self::run_command(
                work_dir, 
                &format!("checkout -b {} origin/{}", branch, branch)
            ).await;
            
            if has_error2 {
                let (has_error3, _) = Self::run_command(work_dir, &format!("checkout {}", branch)).await;
                if !has_error3 {
                    Self::run_command(work_dir, "pull").await;
                }
                return !has_error3;
            }
        }
        
        Self::run_command(work_dir, "pull").await;
        
        !has_error
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

        if has_error {
            return None;
        }

        let result = String::from_utf8_lossy(&output.stdout);
        
        let mut seen_versions = std::collections::HashSet::new();
        let mut tags: Vec<(String, String)> = Vec::new();
        
        let has_dev = Self::has_remote_branch(work_dir, "dev").await;
        let has_qa = Self::has_remote_branch(work_dir, "qa").await;
        
        if has_dev {
            tags.push(("BRANCH:dev".to_string(), "dev-latest".to_string()));
            seen_versions.insert("dev-latest".to_string());
        }
        if has_qa {
            tags.push(("BRANCH:qa".to_string(), "qa-latest".to_string()));
            seen_versions.insert("qa-latest".to_string());
        }
        
        let tag_entries: Vec<(String, String)> = result
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|t| !t.is_empty())
            .filter(|tag| {
                let lower = tag.to_lowercase();
                lower.contains("dev") || lower.contains("qa")
            })
            .map(|tag| {
                let display = Self::format_tag_display(&tag);
                (tag, display)
            })
            .filter(|(_, display)| seen_versions.insert(display.clone()))
            .take(limit.saturating_sub(tags.len()))
            .collect();
        
        tags.extend(tag_entries);
        
        Some(tags)
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