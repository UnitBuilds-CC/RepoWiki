use std::collections::HashMap;
use std::path::Path;

use repowiki_cache::content_hash;
use repowiki_wiki::Wiki;
use serde::{Deserialize, Serialize};

const STATE_FILENAME: &str = ".repowiki-state.json";
const STATE_VERSION: u32 = 1;

#[derive(Debug, Clone, Default)]
pub struct ExportSummary {
    pub written: Vec<String>,
    pub kept: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StateFile {
    version: u32,
    #[serde(default)]
    model: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    pages: HashMap<String, PageState>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PageState {
    inputs: String,
}

fn load_state(output_dir: &Path) -> Option<StateFile> {
    let path = output_dir.join(STATE_FILENAME);
    let text = std::fs::read_to_string(&path).ok()?;
    let state: StateFile = serde_json::from_str(&text).ok()?;
    if state.version != STATE_VERSION {
        return None;
    }
    Some(state)
}

fn save_state(output_dir: &Path, state: &StateFile) -> std::io::Result<()> {
    let path = output_dir.join(STATE_FILENAME);
    let text = serde_json::to_string_pretty(state)?;
    std::fs::write(&path, text + "\n")
}

pub fn export_markdown(
    wiki: &Wiki,
    output_dir: &Path,
    page_inputs: Option<&HashMap<String, String>>,
    model: &str,
    language: &str,
    full: bool,
) -> Option<ExportSummary> {
    std::fs::create_dir_all(output_dir).ok()?;

    let incremental = page_inputs.is_some();
    let state = if incremental && !full {
        load_state(output_dir)
    } else {
        None
    };
    let old_pages = state.as_ref().map(|s| &s.pages);

    let mut summary = ExportSummary::default();
    let mut new_pages: HashMap<String, PageState> = HashMap::new();

    for page in &wiki.pages {
        let page_path = output_dir.join(format!("{}.md", page.id));
        if incremental {
            let inputs = page_inputs
                .and_then(|pi| pi.get(&page.id).cloned())
                .unwrap_or_else(|| format!("page:{}", content_hash(&page.content)));
            new_pages.insert(page.id.clone(), PageState { inputs: inputs.clone() });

            if let Some(old) = old_pages.and_then(|op| op.get(&page.id)) {
                if old.inputs == inputs && page_path.exists() {
                    summary.kept.push(page.id.clone());
                    continue;
                }
            }
        }

        if let Some(parent) = page_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&page_path, &page.content).ok();
        summary.written.push(page.id.clone());
    }

    if incremental {
        if let Some(old) = old_pages {
            for old_id in old.keys() {
                if !new_pages.contains_key(old_id) {
                    let stale = output_dir.join(format!("{}.md", old_id));
                    if stale.exists() {
                        std::fs::remove_file(&stale).ok();
                    }
                    summary.removed.push(old_id.clone());
                }
            }
        }
    }

    write_if_changed(&output_dir.join("_sidebar.md"), &sidebar_text(wiki));
    write_if_changed(&output_dir.join("README.md"), &readme_text(wiki));

    if incremental {
        let new_state = StateFile {
            version: STATE_VERSION,
            model: model.to_string(),
            language: language.to_string(),
            pages: new_pages,
        };
        if Some(&new_state) != state.as_ref() {
            save_state(output_dir, &new_state).ok();
        }
        Some(summary)
    } else {
        None
    }
}

impl PartialEq for StateFile {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.model == other.model
            && self.language == other.language
            && self.pages.len() == other.pages.len()
            && self.pages.iter().all(|(k, v)| other.pages.get(k) == Some(v))
    }
}

fn write_if_changed(path: &Path, content: &str) {
    if let Ok(existing) = std::fs::read_to_string(path) {
        if existing == content {
            return;
        }
    }
    std::fs::write(path, content).ok();
}

fn sidebar_text(wiki: &Wiki) -> String {
    let mut lines = vec![format!("# {}\n", wiki.project_name)];
    for item in &wiki.sidebar {
        if !item.page_id.is_empty() {
            lines.push(format!("- [{}]({}.md)", item.title, item.page_id));
        } else {
            lines.push(format!("- **{}**", item.title));
        }
        for child in &item.children {
            lines.push(format!("  - [{}]({}.md)", child.title, child.page_id));
        }
    }
    lines.join("\n") + "\n"
}

fn readme_text(wiki: &Wiki) -> String {
    let mut lines = vec![format!("# {}\n", wiki.project_name)];
    if let Some(overview) = wiki.get_page("index") {
        let trimmed = overview.content.trim();
        if !trimmed.is_empty() {
            lines.push(trimmed.to_string());
            lines.push(String::new());
        }
    }
    lines.push("## Contents\n".into());
    for item in &wiki.sidebar {
        if !item.page_id.is_empty() {
            lines.push(format!("- [{}]({}.md)", item.title, item.page_id));
        } else {
            lines.push(format!("- **{}**", item.title));
        }
        for child in &item.children {
            lines.push(format!("  - [{}]({}.md)", child.title, child.page_id));
        }
    }
    lines.join("\n") + "\n"
}
