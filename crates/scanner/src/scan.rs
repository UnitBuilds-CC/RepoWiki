use std::collections::HashSet;
use std::path::Path;

use repowiki_core::models::{FileInfo, ScanReport};
use tracing::info;
use walkdir::WalkDir;

use crate::ignore_rules::IgnoreRules;

const SKIP_DIRS: &[&str] = &[
    ".git", "node_modules", "__pycache__", ".venv", "venv", "env",
    ".idea", ".vscode", ".next", "dist", "build", ".tox", ".mypy_cache",
    ".pytest_cache", ".ruff_cache", ".turbo", "coverage",
    ".cache", "vendor", "target", "__snapshots__", ".svn", ".hg",
    ".gradle", ".m2", "Pods", ".dart_tool", ".pub-cache",
];

const SKIP_EXTS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".webp",
    ".mp3", ".mp4", ".wav", ".avi", ".mov", ".mkv", ".flac",
    ".zip", ".tar", ".gz", ".bz2", ".7z", ".rar", ".xz",
    ".exe", ".dll", ".so", ".dylib", ".bin", ".dat",
    ".woff", ".woff2", ".ttf", ".eot", ".otf",
    ".pyc", ".pyo", ".class", ".o", ".obj",
    ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
    ".db", ".sqlite", ".sqlite3",
    ".lock",
    ".min.js", ".min.css",
    ".map",
    ".wasm",
];

const SENSITIVE_NAMES: &[&str] = &[
    ".env", ".env.local", ".env.production", ".env.development",
    ".npmrc", ".pypirc", ".netrc", "id_rsa", "id_ed25519", "known_hosts",
];

const MINIFIED_SOURCE_EXTS: &[&str] = &[".js", ".mjs", ".cjs", ".css"];

const CODE_LANGS: &[&str] = &[
    "python", "javascript", "typescript", "jsx", "tsx", "go", "rust", "java",
    "kotlin", "scala", "c", "cpp", "csharp", "ruby", "php", "r", "sql",
    "swift", "lua", "dart", "vue", "svelte", "zig", "shell", "dockerfile", "makefile",
];

const CONFIG_FILES: &[&str] = &[
    "requirements.txt", "setup.py", "setup.cfg", "pyproject.toml",
    "package.json", "Cargo.toml", "go.mod", "go.sum",
    "Makefile", "CMakeLists.txt",
    "Dockerfile", "docker-compose.yml", "docker-compose.yaml",
    ".env.example", "config.py", "config.yaml", "config.json", "config.toml",
    "README.md", "README.rst", "README.txt", "README",
    "tsconfig.json", "vite.config.ts", "vite.config.js",
    "webpack.config.js", "rollup.config.js",
    "Gemfile", "build.gradle", "pom.xml",
    ".eslintrc.json", ".prettierrc",
];

const ENTRYPOINT_NAMES: &[&str] = &[
    "main.py", "app.py", "index.py", "server.py", "run.py", "__main__.py",
    "main.go", "main.rs", "main.ts", "main.js",
    "index.ts", "index.js", "index.tsx", "index.jsx",
    "App.tsx", "App.jsx", "App.vue", "App.svelte",
    "manage.py", "wsgi.py", "asgi.py",
];

const ENTRYPOINT_DIRS: &[&str] = &["cmd", "bin", "scripts", "entrypoints"];

fn lang_map() -> &'static [(&'static str, &'static str)] {
    &[
        (".py", "python"), (".pyi", "python"),
        (".js", "javascript"), (".mjs", "javascript"), (".cjs", "javascript"),
        (".ts", "typescript"), (".mts", "typescript"),
        (".jsx", "jsx"), (".tsx", "tsx"),
        (".html", "html"), (".htm", "html"),
        (".css", "css"), (".scss", "scss"), (".less", "less"),
        (".json", "json"), (".jsonc", "json"),
        (".yaml", "yaml"), (".yml", "yaml"),
        (".toml", "toml"),
        (".md", "markdown"), (".mdx", "markdown"),
        (".txt", "text"), (".rst", "rst"),
        (".sh", "shell"), (".bash", "shell"), (".zsh", "shell"),
        (".go", "go"),
        (".rs", "rust"),
        (".java", "java"),
        (".kt", "kotlin"), (".kts", "kotlin"),
        (".scala", "scala"),
        (".c", "c"), (".h", "c"),
        (".cpp", "cpp"), (".hpp", "cpp"), (".cc", "cpp"), (".cxx", "cpp"),
        (".cs", "csharp"),
        (".rb", "ruby"),
        (".php", "php"),
        (".r", "r"), (".R", "r"),
        (".sql", "sql"),
        (".swift", "swift"),
        (".lua", "lua"),
        (".dart", "dart"),
        (".vue", "vue"),
        (".svelte", "svelte"),
        (".zig", "zig"),
        (".nim", "nim"),
        (".ex", "elixir"), (".exs", "elixir"),
        (".erl", "erlang"),
        (".hs", "haskell"),
        (".ml", "ocaml"),
        (".clj", "clojure"),
        (".proto", "protobuf"),
        (".graphql", "graphql"), (".gql", "graphql"),
        (".tf", "terraform"), (".hcl", "hcl"),
        (".prisma", "prisma"),
        (".astro", "astro"),
        (".cfg", "ini"), (".ini", "ini"),
        (".env", "text"),
        (".cmake", "cmake"),
        (".gradle", "gradle"),
        (".dockerfile", "dockerfile"),
    ]
}

pub fn detect_language(path: &str) -> String {
    let name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    if name == "dockerfile" || name.starts_with("dockerfile.") {
        return "dockerfile".into();
    }
    if name == "makefile" {
        return "makefile".into();
    }
    let ext = Path::new(path)
        .extension()
        .map(|e| format!(".{}", e.to_str().unwrap_or("")))
        .unwrap_or_default()
        .to_lowercase();
    for (e, lang) in lang_map() {
        if *e == ext {
            return lang.to_string();
        }
    }
    "unknown".into()
}

fn is_binary(data: &[u8]) -> bool {
    data[..data.len().min(1024)].contains(&0)
}

fn has_skipped_suffix(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    SKIP_EXTS.iter().any(|ext| name.ends_with(ext))
}

fn is_sensitive_name(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    if SENSITIVE_NAMES.contains(&name.as_str()) {
        return true;
    }
    name.starts_with(".env.") && name != ".env.example"
}

fn looks_minified_source(path: &str, text: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .map(|e| format!(".{}", e.to_str().unwrap_or("")))
        .unwrap_or_default()
        .to_lowercase();
    if !MINIFIED_SOURCE_EXTS.contains(&ext.as_str()) {
        return false;
    }
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return false;
    }
    let longest = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    if longest < 1000 {
        return false;
    }
    let non_empty = lines.iter().filter(|l| !l.trim().is_empty()).count();
    non_empty <= 5 || longest > text.len() / 2
}

fn is_entrypoint(rel_path: &str) -> bool {
    let path = Path::new(rel_path);
    let parts: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    if let Some(name) = parts.last() {
        if ENTRYPOINT_NAMES.contains(name) {
            return true;
        }
    }
    if parts.len() >= 2 {
        if ENTRYPOINT_DIRS.contains(&parts[parts.len() - 2]) {
            return true;
        }
    }
    false
}

pub fn build_file_tree(files: &[FileInfo], max_lines: usize) -> String {
    let mut entries: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for f in files {
        let path_normalized = f.path.replace('\\', "/");
        if seen.insert(path_normalized.clone()) {
            entries.push(path_normalized.clone());
        }
        let parts: Vec<&str> = path_normalized.split('/').collect();
        for i in 1..parts.len() {
            let dir = parts[..i].join("/") + "/";
            if seen.insert(dir.clone()) {
                entries.push(dir);
            }
        }
    }

    entries.sort();
    let mut lines: Vec<String> = Vec::new();
    for entry in entries.iter().take(max_lines) {
        let stripped = entry.trim_end_matches('/');
        let depth = stripped.matches('/').count();
        let indent = "  ".repeat(depth);
        let name = stripped.rsplit('/').next().unwrap_or(stripped);
        let display = if entry.ends_with('/') {
            format!("{name}/")
        } else {
            name.to_string()
        };
        lines.push(format!("{indent}{display}"));
    }
    if entries.len() > max_lines {
        lines.push(format!(
            "  ... and {} more entries",
            entries.len() - max_lines
        ));
    }
    lines.join("\n")
}

pub fn scan_directory(
    root: &Path,
    max_file_size: u64,
    max_files: u32,
    preview_lines: usize,
    mut report: Option<&mut ScanReport>,
    exclude_dirs: &[String],
) -> std::io::Result<Vec<FileInfo>> {
    let root = root.canonicalize()?;
    if !root.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Not a directory: {}", root.display()),
        ));
    }

    let ignore_rules = IgnoreRules::from_root(&root);
    let hard_cap = std::cmp::max(max_files as usize * 4, max_files as usize + 100);

    let mut results: Vec<FileInfo> = Vec::new();
    let skip_dirs: HashSet<&str> = SKIP_DIRS.iter().copied().collect();

    for entry in WalkDir::new(&root).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        if results.len() >= hard_cap {
            info!("Hit hard scan cap ({hard_cap}), stopping");
            break;
        }

        let full = entry.path();
        let rel = full.strip_prefix(&root).unwrap_or(full);
        let rel_posix = rel.to_string_lossy().replace('\\', "/");

        // check parent dirs against skip dirs
        let mut skip = false;
        for component in rel.components() {
            let comp = component.as_os_str().to_str().unwrap_or("");
            if skip_dirs.contains(comp) || comp.ends_with(".egg-info") {
                skip = true;
                break;
            }
            if exclude_dirs.iter().any(|d| d == comp) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        if ignore_rules.matches(&rel_posix, false) {
            continue;
        }
        if is_sensitive_name(full) {
            continue;
        }
        if has_skipped_suffix(full) {
            continue;
        }

        if let Some(rpt) = report.as_deref_mut() {
            rpt.candidates += 1;
        }

        let meta = match full.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let size = meta.len();
        if size > max_file_size || size == 0 {
            if let Some(rpt) = report.as_deref_mut() {
                if size > max_file_size {
                    rpt.oversized_count += 1;
                    if rpt.oversized.len() < 3 {
                        rpt.oversized.push(rel_posix.clone());
                    }
                }
            }
            continue;
        }

        let raw = match std::fs::read(full) {
            Ok(d) => d,
            Err(_) => continue,
        };
        if is_binary(&raw) {
            if let Some(rpt) = report.as_deref_mut() {
                rpt.binary_count += 1;
            }
            continue;
        }

        let text = match String::from_utf8(raw) {
            Ok(s) => s,
            Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
        };

        if looks_minified_source(&rel_posix, &text) {
            if let Some(rpt) = report.as_deref_mut() {
                rpt.minified_count += 1;
            }
            continue;
        }

        let lang = detect_language(&rel_posix);
        let fname = full
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let is_cfg = CONFIG_FILES.contains(&fname);
        let is_entry = is_entrypoint(&rel_posix);
        let line_count = text.matches('\n').count() as u32 + 1;

        let preview = if is_cfg || is_entry {
            text.clone()
        } else {
            text.lines().take(preview_lines).collect::<Vec<_>>().join("\n")
        };

        results.push(FileInfo {
            path: rel_posix,
            size,
            language: lang,
            lines: line_count,
            preview,
            content: text,
            is_config: is_cfg,
            is_entrypoint: is_entry,
        });
    }

    // priority-based file cap
    if results.len() > max_files as usize {
        let mut indexed: Vec<(usize, u8)> = results
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let priority = if f.is_config || f.is_entrypoint {
                    0
                } else if CODE_LANGS.contains(&f.language.as_str()) {
                    1
                } else {
                    2
                };
                (i, priority)
            })
            .collect();
        indexed.sort_by_key(|&(i, p)| (p, i));
        let keep_indices: Vec<usize> = indexed
            .iter()
            .take(max_files as usize)
            .map(|&(i, _)| i)
            .collect();
        let dropped = results.len() - keep_indices.len();
        let mut keep_set: Vec<usize> = keep_indices.clone();
        keep_set.sort();
        results = keep_set.into_iter().map(|i| results[i].clone()).collect();
        if let Some(rpt) = report.as_deref_mut() {
            rpt.priority_dropped = dropped as u32;
        }
        info!(
            "File cap ({}) hit; dropped {} lower-priority files",
            max_files, dropped
        );
    }

    if let Some(rpt) = report.as_deref_mut() {
        rpt.kept = results.len() as u32;
    }

    results.sort_by(|a, b| {
        let ka = sort_key(a);
        let kb = sort_key(b);
        ka.cmp(&kb)
    });

    Ok(results)
}

fn sort_key(f: &FileInfo) -> (u8, &str) {
    if f.is_config {
        (0, &f.path)
    } else if f.is_entrypoint {
        (1, &f.path)
    } else {
        (2, &f.path)
    }
}
