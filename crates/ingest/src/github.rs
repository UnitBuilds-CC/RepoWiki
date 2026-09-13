use std::path::PathBuf;
use std::process::Command;

use regex::Regex;
use tracing::info;

use crate::local::ingest_local;
use repowiki_core::models::ProjectContext;

const MAX_REPO_SIZE_MB: u64 = 500;

fn clone_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".repowiki")
        .join("repos")
}

fn git_url_regex() -> Regex {
    Regex::new(
        r"(?:https?://)?(?:www\.)?(github\.com|gitlab\.com|bitbucket\.org)/([^/\s]+)/([^/\s#?.]+)",
    )
    .unwrap()
}

pub fn parse_git_url(url: &str) -> Option<(String, String, String)> {
    let url = url.trim().trim_end_matches('/');
    let url = url.strip_suffix(".git").unwrap_or(url);

    let re = git_url_regex();
    let caps = re.captures(url)?;
    let host = caps.get(1)?.as_str().to_string();
    let owner = caps.get(2)?.as_str().to_string();
    let repo = caps.get(3)?.as_str().to_string();
    Some((host, owner, repo))
}

fn clone_url(url: &str) -> String {
    match parse_git_url(url) {
        Some((host, owner, repo)) => format!("https://{}/{}/{}.git", host, owner, repo),
        None => url.to_string(),
    }
}

fn resolve_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .or_else(|| std::env::var("GH_TOKEN").ok())
}

fn authenticated_clone_url(url: &str) -> (String, Option<String>) {
    let base = clone_url(url);
    let token = resolve_token();

    if let Some(ref tok) = token {
        if let Some((host, owner, repo)) = parse_git_url(url) {
            if host == "github.com" {
                return (
                    format!(
                        "https://x-access-token:{}@github.com/{}/{}.git",
                        tok, owner, repo
                    ),
                    Some(tok.clone()),
                );
            }
        }
    }

    (base, None)
}

fn remove_dir_all(path: &std::path::Path) -> std::io::Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

fn dir_size_mb(path: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                total += p.metadata().map(|m| m.len()).unwrap_or(0);
            } else if p.is_dir() {
                total += dir_size_mb(&p);
            }
        }
    }
    total / (1024 * 1024)
}

pub fn ingest_github(
    url: &str,
    max_file_size: u64,
    max_files: u32,
    force_reclone: bool,
    exclude_dirs: &[String],
) -> Result<ProjectContext, String> {
    let (host, owner, repo) =
        parse_git_url(url).ok_or_else(|| format!("Can't parse git URL: {}", url))?;

    let dest = clone_dir().join(&host).join(&owner).join(&repo);

    if dest.exists() {
        if force_reclone {
            remove_dir_all(&dest).map_err(|e| format!("Failed to remove cached clone: {}", e))?;
        } else {
            info!("Using cached clone: {}", dest.display());
            return ingest_local(&dest, max_file_size, max_files, exclude_dirs)
                .map_err(|e| format!("Failed to ingest cached clone: {}", e));
        }
    }

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create clone directory: {}", e))?;
    }

    let (auth_url, token) = authenticated_clone_url(url);
    let display_url = clone_url(url);
    info!("Cloning {} -> {}", display_url, dest.display());

    let output = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--single-branch",
            &auth_url,
            &dest.to_string_lossy(),
        ])
        .output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                let _ = remove_dir_all(&dest);
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let stderr = match &token {
                    Some(tok) => stderr.replace(tok, "***"),
                    None => stderr,
                };
                return Err(format!("Clone failed: {}", stderr));
            }
        }
        Err(e) => {
            let _ = remove_dir_all(&dest);
            return Err(format!("Failed to run git: {}", e));
        }
    }

    let total_mb = dir_size_mb(&dest);
    if total_mb > MAX_REPO_SIZE_MB {
        let _ = remove_dir_all(&dest);
        return Err(format!(
            "Repo too large ({} MB > {} MB limit)",
            total_mb, MAX_REPO_SIZE_MB
        ));
    }

    ingest_local(&dest, max_file_size, max_files, exclude_dirs)
        .map_err(|e| format!("Failed to ingest cloned repo: {}", e))
}
