use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

const DEFAULT_TTL: f64 = 365.0 * 24.0 * 3600.0;

fn cache_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".repowiki")
}

fn default_db_path() -> PathBuf {
    cache_dir().join("cache.db")
}

pub fn content_hash(content: &str) -> String {
    let hash = Sha256::digest(content.as_bytes());
    hex::encode(&hash[..12])
}

fn now_secs() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[derive(Clone)]
pub struct Cache {
    conn: Arc<Mutex<Connection>>,
}

impl Cache {
    pub fn open(db_path: Option<&Path>) -> Result<Self, CacheError> {
        let path = db_path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(default_db_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cache \
             (key TEXT PRIMARY KEY, value TEXT, created_at REAL);
             CREATE TABLE IF NOT EXISTS projects \
             (id TEXT PRIMARY KEY, data TEXT, created_at REAL);",
        )?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn get(&self, key: &str, ttl: f64) -> Option<serde_json::Value> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT value, created_at FROM cache WHERE key = ?1",
            params![key],
            |row| {
                let value: String = row.get(0)?;
                let created_at: f64 = row.get(1)?;
                Ok((value, created_at))
            },
        );
        match result {
            Ok((value, created_at)) => {
                if now_secs() - created_at > ttl {
                    let _ = conn.execute("DELETE FROM cache WHERE key = ?1", params![key]);
                    return None;
                }
                serde_json::from_str(&value).ok()
            }
            Err(_) => None,
        }
    }

    pub fn get_default_ttl(&self, key: &str) -> Option<serde_json::Value> {
        self.get(key, DEFAULT_TTL)
    }

    pub fn put(&self, key: &str, value: &serde_json::Value) -> Result<(), CacheError> {
        let json = serde_json::to_string(value)?;
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO cache (key, value, created_at) VALUES (?1, ?2, ?3)",
            params![key, json, now_secs()],
        )?;
        Ok(())
    }

    pub fn save_project(&self, project_id: &str, data: &serde_json::Value) -> Result<(), CacheError> {
        let json = serde_json::to_string(data)?;
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO projects (id, data, created_at) VALUES (?1, ?2, ?3)",
            params![project_id, json, now_secs()],
        )?;
        Ok(())
    }

    pub fn load_project(&self, project_id: &str) -> Option<serde_json::Value> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT data FROM projects WHERE id = ?1",
            params![project_id],
            |row| {
                let data: String = row.get(0)?;
                Ok(data)
            },
        );
        result.ok().and_then(|s| serde_json::from_str(&s).ok())
    }

    pub fn clear(&self) -> Result<usize, CacheError> {
        let conn = self.conn.lock().unwrap();
        let count = conn.query_row("SELECT COUNT(*) FROM cache", [], |row| {
            row.get::<_, usize>(0)
        })?;
        conn.execute("DELETE FROM cache", [])?;
        Ok(count)
    }
}
