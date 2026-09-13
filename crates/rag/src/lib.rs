use std::collections::HashMap;
use std::path::{Path, PathBuf};

use regex::Regex;
use repowiki_core::models::ProjectContext;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const INDEX_DIR_NAME: &str = "rag";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub file_path: String,
    pub line_start: u32,
    pub line_end: u32,
    pub content: String,
    #[serde(default)]
    pub score: f64,
}

fn default_index_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".repowiki")
        .join(INDEX_DIR_NAME)
}

pub fn index_fingerprint(project: &ProjectContext) -> String {
    let mut hasher = Sha256::new();
    hasher.update(project.root.as_bytes());
    let mut sorted_files = project.files.clone();
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));
    for f in &sorted_files {
        let text = if !f.content.is_empty() {
            &f.content
        } else {
            &f.preview
        };
        hasher.update(f.path.as_bytes());
        hasher.update(f.size.to_string().as_bytes());
        let content_hash = Sha256::digest(text.as_bytes());
        hasher.update(&content_hash);
    }
    let hash = hasher.finalize();
    hex::encode(&hash[..12])
}

pub struct SimpleRAG {
    pub chunks: Vec<Chunk>,
    idf: HashMap<String, f64>,
    tf_vectors: Vec<HashMap<String, u32>>,
}

impl SimpleRAG {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            idf: HashMap::new(),
            tf_vectors: Vec::new(),
        }
    }

    pub fn index(&mut self, project: &ProjectContext) {
        self.chunks.clear();
        for f in &project.files {
            let text = if !f.content.is_empty() {
                &f.content
            } else {
                &f.preview
            };
            if text.is_empty() {
                continue;
            }
            let file_chunks = split_into_chunks(text, &f.path, 30);
            self.chunks.extend(file_chunks);
        }

        let doc_count = self.chunks.len();
        if doc_count == 0 {
            return;
        }

        let mut df: HashMap<String, u32> = HashMap::new();
        self.tf_vectors.clear();

        for chunk in &self.chunks {
            let tokens = tokenize(&chunk.content);
            let mut tf: HashMap<String, u32> = HashMap::new();
            for token in &tokens {
                *tf.entry(token.clone()).or_default() += 1;
            }
            for token in tf.keys() {
                *df.entry(token.clone()).or_default() += 1;
            }
            self.tf_vectors.push(tf);
        }

        let dc = doc_count as f64;
        self.idf = df
            .into_iter()
            .map(|(token, count)| (token, (dc / (count as f64 + 1.0)).ln()))
            .collect();
    }

    pub fn save_index(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let payload = IndexPayload {
            chunks: self
                .chunks
                .iter()
                .map(|c| ChunkData {
                    file_path: c.file_path.clone(),
                    line_start: c.line_start,
                    line_end: c.line_end,
                    content: c.content.clone(),
                })
                .collect(),
            idf: self.idf.clone(),
            tf_vectors: self.tf_vectors.clone(),
        };
        let tmp = path.with_extension("tmp");
        let json = serde_json::to_string(&payload).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, path)
    }

    pub fn load_index(path: &Path) -> Option<Self> {
        let data = std::fs::read_to_string(path).ok()?;
        let payload: IndexPayload = serde_json::from_str(&data).ok()?;
        let rag = Self {
            chunks: payload
                .chunks
                .into_iter()
                .map(|c| Chunk {
                    file_path: c.file_path,
                    line_start: c.line_start,
                    line_end: c.line_end,
                    content: c.content,
                    score: 0.0,
                })
                .collect(),
            idf: payload.idf,
            tf_vectors: payload.tf_vectors,
        };
        if rag.chunks.len() != rag.tf_vectors.len() {
            return None;
        }
        Some(rag)
    }

    pub fn retrieve(&self, query: &str, top_k: usize) -> Vec<Chunk> {
        if self.chunks.is_empty() {
            return Vec::new();
        }

        let query_tokens = tokenize(query);
        let mut query_tf: HashMap<String, u32> = HashMap::new();
        for token in &query_tokens {
            *query_tf.entry(token.clone()).or_default() += 1;
        }

        let mut scores: Vec<(f64, usize)> = self
            .chunks
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let score = cosine_similarity(&query_tf, &self.tf_vectors[i], &self.idf);
                (score, i)
            })
            .collect();

        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        scores
            .into_iter()
            .take(top_k)
            .filter(|(score, _)| *score > 0.0)
            .map(|(score, idx)| {
                let mut chunk = self.chunks[idx].clone();
                chunk.score = score;
                chunk
            })
            .collect()
    }
}

pub fn load_or_build_index(
    project: &ProjectContext,
    index_dir: Option<&Path>,
) -> (SimpleRAG, bool) {
    let dir = index_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_index_dir);
    let path = dir.join(format!("{}.json", index_fingerprint(project)));
    if let Some(rag) = SimpleRAG::load_index(&path) {
        if !rag.chunks.is_empty() {
            return (rag, true);
        }
    }
    let mut rag = SimpleRAG::new();
    rag.index(project);
    if !rag.chunks.is_empty() {
        let _ = rag.save_index(&path);
    }
    (rag, false)
}

pub fn format_context(chunks: &[Chunk]) -> String {
    if chunks.is_empty() {
        return "(no relevant code found in this repository)".into();
    }
    chunks
        .iter()
        .map(|c| {
            format!(
                "### {} (lines {}-{})\n```\n{}\n```",
                c.file_path, c.line_start, c.line_end, c.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    re.find_iter(&text.to_lowercase())
        .map(|m| m.as_str().to_string())
        .collect()
}

fn cosine_similarity(
    vec_a: &HashMap<String, u32>,
    vec_b: &HashMap<String, u32>,
    idf: &HashMap<String, f64>,
) -> f64 {
    let common: Vec<&String> = vec_a.keys().filter(|k| vec_b.contains_key(*k)).collect();
    if common.is_empty() {
        return 0.0;
    }

    let dot: f64 = common
        .iter()
        .map(|t| {
            let a = *vec_a.get(*t).unwrap_or(&0) as f64 * idf.get(*t).copied().unwrap_or(0.0);
            let b = *vec_b.get(*t).unwrap_or(&0) as f64 * idf.get(*t).copied().unwrap_or(0.0);
            a * b
        })
        .sum();

    let norm_a: f64 = vec_a
        .iter()
        .map(|(t, &c)| {
            let v = c as f64 * idf.get(t).copied().unwrap_or(0.0);
            v * v
        })
        .sum::<f64>()
        .sqrt();

    let norm_b: f64 = vec_b
        .iter()
        .map(|(t, &c)| {
            let v = c as f64 * idf.get(t).copied().unwrap_or(0.0);
            v * v
        })
        .sum::<f64>()
        .sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

fn split_into_chunks(text: &str, file_path: &str, max_chunk_lines: usize) -> Vec<Chunk> {
    let lines: Vec<&str> = text.lines().collect();
    let mut chunks = Vec::new();
    let mut current_start = 0usize;
    let mut current_lines: Vec<&str> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        current_lines.push(line);

        let is_boundary = line.trim().is_empty() && current_lines.len() >= 5;
        let is_too_long = current_lines.len() >= max_chunk_lines;

        if is_boundary || is_too_long || i == lines.len() - 1 {
            if !current_lines.is_empty() {
                let content = current_lines.join("\n");
                if !content.trim().is_empty() {
                    chunks.push(Chunk {
                        file_path: file_path.to_string(),
                        line_start: (current_start + 1) as u32,
                        line_end: (current_start + current_lines.len()) as u32,
                        content,
                        score: 0.0,
                    });
                }
                current_start = i + 1;
                current_lines.clear();
            }
        }
    }

    chunks
}

#[derive(Serialize, Deserialize)]
struct IndexPayload {
    chunks: Vec<ChunkData>,
    idf: HashMap<String, f64>,
    tf_vectors: Vec<HashMap<String, u32>>,
}

#[derive(Serialize, Deserialize)]
struct ChunkData {
    file_path: String,
    line_start: u32,
    line_end: u32,
    content: String,
}
