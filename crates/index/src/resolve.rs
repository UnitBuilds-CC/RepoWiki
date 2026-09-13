use std::collections::HashSet;

use repowiki_core::models::IndexedImport;

pub fn resolve_imports(
    imports: &mut Vec<IndexedImport>,
    source_file: &str,
    language: &str,
    known_paths: &HashSet<&str>,
) {
    for imp in imports.iter_mut() {
        if let Some(resolved) = resolve_one(&imp.raw, source_file, language, known_paths) {
            imp.resolved_path = Some(resolved);
        }
    }
}

fn resolve_one(
    raw: &str,
    source_file: &str,
    language: &str,
    known_paths: &HashSet<&str>,
) -> Option<String> {
    let import_path = extract_module_path(raw, language)?;
    let candidates = build_candidates(&import_path, source_file, language);
    for c in candidates {
        let normalized = normalize_path(&c.replace('\\', "/"));
        if known_paths.contains(normalized.as_str()) {
            return Some(normalized);
        }
    }
    None
}

fn extract_module_path(raw: &str, language: &str) -> Option<String> {
    let trimmed = raw.trim();
    match language {
        "python" | "pyi" => {
            if let Some(rest) = trimmed.strip_prefix("from ") {
                rest.split_whitespace().next().map(|s| s.to_string())
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                Some(rest.split_whitespace().next().unwrap_or("").to_string())
            } else {
                Some(trimmed.to_string())
            }
        }
        "rust" => {
            let s = trimmed.strip_prefix("use ").unwrap_or(trimmed);
            let s = s.split('{').next().unwrap_or(s).trim();
            let s = s.split(" as ").next().unwrap_or(s).trim();
            Some(s.to_string())
        }
        "go" => Some(trimmed.to_string()),
        _ => Some(trimmed.to_string()),
    }
}

fn build_candidates(import_path: &str, source_file: &str, language: &str) -> Vec<String> {
    match language {
        "python" | "pyi" => {
            let rel = resolve_python_module(import_path, source_file);
            let mut c = vec![
                format!("{rel}.py"),
                format!("{rel}/__init__.py"),
            ];
            if !rel.starts_with("src/") {
                c.push(format!("src/{rel}.py"));
                c.push(format!("src/{rel}/__init__.py"));
            }
            c
        }
        "javascript" | "typescript" | "jsx" | "tsx" | "mjs" | "cjs" | "mts" => {
            let rel = if import_path.starts_with('.') {
                let base_dir = source_file.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
                normalize_path(&format!("{base_dir}/{import_path}"))
            } else {
                import_path.to_string()
            };
            vec![
                rel.clone(),
                format!("{rel}.ts"),
                format!("{rel}.tsx"),
                format!("{rel}.js"),
                format!("{rel}.jsx"),
                format!("{rel}/index.ts"),
                format!("{rel}/index.tsx"),
                format!("{rel}/index.js"),
                format!("{rel}/index.jsx"),
            ]
        }
        "go" => {
            let parts: Vec<&str> = import_path.split('/').collect();
            if parts.len() >= 2 {
                vec![format!(
                    "{}/{}.go",
                    parts[parts.len() - 2],
                    parts[parts.len() - 1]
                )]
            } else {
                vec![]
            }
        }
        "rust" => {
            let rel = import_path.split("::").next().unwrap_or("").replace("::", "/");
            vec![
                format!("src/{rel}.rs"),
                format!("src/{rel}/mod.rs"),
                format!("{rel}.rs"),
            ]
        }
        "java" => {
            let rel = import_path.replace('.', "/");
            vec![format!("src/main/java/{rel}.java"), format!("{rel}.java")]
        }
        _ => vec![],
    }
}

fn resolve_python_module(import_path: &str, source_file: &str) -> String {
    let leading_dots = import_path.chars().take_while(|&c| c == '.').count();
    let module = import_path[leading_dots..].replace('.', "/");
    if leading_dots == 0 {
        return module;
    }

    let source_normalized = source_file.replace('\\', "/");
    let source_dir: Vec<&str> = source_normalized
        .rsplit_once('/')
        .map(|(d, _)| d.split('/').collect())
        .unwrap_or_default();
    let keep = (source_dir.len() as i32 - leading_dots as i32 + 1).max(0) as usize;
    let mut parts: Vec<String> = source_dir[..keep].iter().map(|s| s.to_string()).collect();
    if !module.is_empty() {
        parts.extend(module.split('/').map(|s| s.to_string()));
    }
    parts.join("/")
}

fn normalize_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}
