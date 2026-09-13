use std::path::Path;

use repowiki_core::models::{FileInfo, ProjectContext, ScanReport};
use repowiki_scanner::{build_file_tree, scan_directory};

fn guess_project_name(root: &Path, files: &[FileInfo]) -> String {
    for f in files {
        if f.path == "pyproject.toml" && !f.content.is_empty() {
            for line in f.content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("name") {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        let val = val.trim().trim_matches('"').trim_matches('\'');
                        if !val.is_empty() {
                            return val.to_string();
                        }
                    }
                }
            }
        }

        if f.path == "package.json" && !f.content.is_empty() {
            if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&f.content) {
                if let Some(name) = pkg.get("name").and_then(|n| n.as_str()) {
                    return name.trim_start_matches('@').replace('/', "-");
                }
            }
        }

        if f.path == "Cargo.toml" && !f.content.is_empty() {
            for line in f.content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("name") {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        let val = val.trim().trim_matches('"').trim_matches('\'');
                        if !val.is_empty() {
                            return val.to_string();
                        }
                    }
                }
            }
        }
    }

    root.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

pub fn ingest_local(
    path: impl AsRef<Path>,
    max_file_size: u64,
    max_files: u32,
    exclude_dirs: &[String],
) -> std::io::Result<ProjectContext> {
    let root = path.as_ref().canonicalize()?;
    if !root.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Not a directory: {}", root.display()),
        ));
    }

    let mut report = ScanReport::default();
    let files = scan_directory(&root, max_file_size, max_files, 2, Some(&mut report), exclude_dirs)?;
    let name = guess_project_name(&root, &files);
    let tree = build_file_tree(&files, 2);

    Ok(ProjectContext {
        name,
        root: root.to_string_lossy().into_owned(),
        files,
        file_tree: tree,
        coverage: Some(report),
    })
}
