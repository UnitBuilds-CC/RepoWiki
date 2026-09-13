use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".repowiki")
}

fn config_file() -> PathBuf {
    config_dir().join("config.json")
}

fn model_aliases() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("deepseek", "deepseek/deepseek-chat"),
        ("opus", "anthropic/claude-opus-4-6"),
        ("claude", "anthropic/claude-sonnet-4-6"),
        ("gpt", "gpt-5.4"),
        ("gpt-mini", "gpt-5.4-mini"),
        ("gemini", "gemini/gemini-3.1-pro-preview"),
        ("gemini-flash", "gemini/gemini-2.5-flash"),
        ("qwen", "openai/qwen3.5-plus"),
        ("kimi", "openai/kimi-k2.6"),
        ("glm", "openai/glm-5"),
        ("minimax", "openai/MiniMax-M2.7"),
    ])
}

pub fn resolve_model(name: &str) -> String {
    model_aliases()
        .get(name)
        .map(|s| s.to_string())
        .unwrap_or_else(|| name.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub api_base: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
    #[serde(default = "default_max_files")]
    pub max_files: u32,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_concurrency")]
    pub concurrency: u32,
}

fn default_model() -> String {
    "deepseek/deepseek-chat".into()
}
fn default_language() -> String {
    "en".into()
}
fn default_max_file_size() -> u64 {
    200 * 1024
}
fn default_max_files() -> u32 {
    1000
}
fn default_output_dir() -> String {
    "./wiki".into()
}
fn default_concurrency() -> u32 {
    5
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: default_model(),
            api_key: String::new(),
            api_base: String::new(),
            language: default_language(),
            max_file_size: default_max_file_size(),
            max_files: default_max_files(),
            output_dir: default_output_dir(),
            concurrency: default_concurrency(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    api_key: Option<String>,
    #[serde(default)]
    api_base: Option<String>,
    #[serde(default)]
    language: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let mut cfg = Self::default();

        let path = config_file();
        if path.exists() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(cf) = serde_json::from_str::<ConfigFile>(&data) {
                    if let Some(v) = cf.model {
                        cfg.model = v;
                    }
                    if let Some(v) = cf.api_key {
                        cfg.api_key = v;
                    }
                    if let Some(v) = cf.api_base {
                        cfg.api_base = v;
                    }
                    if let Some(v) = cf.language {
                        cfg.language = v;
                    }
                }
            }
        }

        // env overrides
        if let Ok(v) = std::env::var("REPOWIKI_MODEL") {
            if !v.is_empty() {
                cfg.model = v;
            }
        }
        if let Ok(v) = std::env::var("REPOWIKI_API_KEY") {
            if !v.is_empty() {
                cfg.api_key = v;
            }
        }
        if let Ok(v) = std::env::var("REPOWIKI_API_BASE") {
            if !v.is_empty() {
                cfg.api_base = v;
            }
        }
        if let Ok(v) = std::env::var("REPOWIKI_LANG") {
            if !v.is_empty() {
                cfg.language = v;
            }
        }

        // fall back to common provider keys
        if cfg.api_key.is_empty() {
            for env_key in &["DEEPSEEK_API_KEY", "OPENROUTER_API_KEY", "OPENAI_API_KEY", "ANTHROPIC_API_KEY"] {
                if let Ok(v) = std::env::var(env_key) {
                    if !v.is_empty() {
                        cfg.api_key = v;
                        break;
                    }
                }
            }
        }

        cfg.model = resolve_model(&cfg.model);
        cfg
    }

    pub fn save(&self) -> std::io::Result<()> {
        let dir = config_dir();
        std::fs::create_dir_all(&dir)?;

        let mut map = serde_json::Map::new();
        if !self.model.is_empty() {
            map.insert(
                "model".into(),
                serde_json::Value::String(self.model.clone()),
            );
        }
        if !self.api_key.is_empty() {
            map.insert(
                "api_key".into(),
                serde_json::Value::String(self.api_key.clone()),
            );
        }
        if !self.api_base.is_empty() {
            map.insert(
                "api_base".into(),
                serde_json::Value::String(self.api_base.clone()),
            );
        }
        if !self.language.is_empty() {
            map.insert(
                "language".into(),
                serde_json::Value::String(self.language.clone()),
            );
        }

        let json = serde_json::to_string_pretty(&map).unwrap();
        std::fs::write(dir.join("config.json"), json + "\n")
    }
}
