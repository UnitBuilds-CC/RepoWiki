use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub path: Option<String>,
    pub url: Option<String>,
    #[serde(default = "default_language")]
    pub language: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub total_files: usize,
    pub total_lines: u64,
    pub error: String,
}

impl ProjectInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            status: "pending".to_string(),
            total_files: 0,
            total_lines: 0,
            error: String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub question: String,
    #[serde(default)]
    pub history: Vec<ChatTurn>,
}

#[derive(Debug, Serialize)]
pub struct FileReference {
    pub path: String,
    pub line_start: u32,
    pub line_end: u32,
    pub snippet: String,
}
