use std::sync::{Arc, Mutex};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

fn resolve_api_base(model: &str, api_key: &str, api_base: &str) -> String {
    if !api_base.is_empty() {
        return format!("{}/chat/completions", api_base.trim_end_matches('/'));
    }
    if api_key.starts_with("sk-or-") {
        return "https://openrouter.ai/api/v1/chat/completions".into();
    }
    let provider = model.split('/').next().unwrap_or("");
    match provider {
        "deepseek" => "https://api.deepseek.com/chat/completions".into(),
        "anthropic" => "https://api.anthropic.com/v1/chat/completions".into(),
        "gemini" => "https://generativelanguage.googleapis.com/v1beta/chat/completions".into(),
        _ => "https://api.openai.com/v1/chat/completions".into(),
    }
}

fn resolve_model_name(model: &str, api_key: &str, api_base: &str) -> String {
    if !api_base.is_empty() || api_key.starts_with("sk-or-") {
        return model.to_string();
    }
    model.split('/').nth(1).unwrap_or(model).to_string()
}

#[derive(Clone)]
pub struct LLMClient {
    client: Client,
    model: String,
    api_key: String,
    api_base: String,
    total_input_tokens: Arc<Mutex<u64>>,
    total_output_tokens: Arc<Mutex<u64>>,
    total_cost: Arc<Mutex<f64>>,
}

impl LLMClient {
    pub fn new(model: &str, api_key: &str, api_base: &str) -> Self {
        Self {
            client: Client::new(),
            model: model.to_string(),
            api_key: api_key.to_string(),
            api_base: api_base.to_string(),
            total_input_tokens: Arc::new(Mutex::new(0)),
            total_output_tokens: Arc::new(Mutex::new(0)),
            total_cost: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn total_input_tokens(&self) -> u64 {
        *self.total_input_tokens.lock().unwrap()
    }

    pub fn total_output_tokens(&self) -> u64 {
        *self.total_output_tokens.lock().unwrap()
    }

    pub fn total_cost(&self) -> f64 {
        *self.total_cost.lock().unwrap()
    }

    pub async fn complete(
        &self,
        messages: &[ChatMessage],
        temperature: f64,
        max_tokens: u32,
    ) -> Result<String, LLMError> {
        let url = resolve_api_base(&self.model, &self.api_key, &self.api_base);
        let model_name = resolve_model_name(&self.model, &self.api_key, &self.api_base);

        let body = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens,
        });

        let mut req = self.client.post(&url).json(&body);
        if !self.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(LLMError::Api(format!("{status}: {body}")));
        }

        let chat_resp: ChatResponse = resp.json().await?;

        if let Some(usage) = &chat_resp.usage {
            *self.total_input_tokens.lock().unwrap() += usage.prompt_tokens.unwrap_or(0);
            *self.total_output_tokens.lock().unwrap() += usage.completion_tokens.unwrap_or(0);
        }

        Ok(chat_resp
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    pub async fn stream(
        &self,
        messages: &[ChatMessage],
        temperature: f64,
        max_tokens: u32,
    ) -> Result<Vec<String>, LLMError> {
        let url = resolve_api_base(&self.model, &self.api_key, &self.api_base);
        let model_name = resolve_model_name(&self.model, &self.api_key, &self.api_base);

        let body = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "stream": true,
        });

        let mut req = self.client.post(&url).json(&body);
        if !self.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(LLMError::Api(format!("{status}: {body}")));
        }

        let text = resp.text().await?;
        let mut chunks = Vec::new();
        for line in text.lines() {
            let line = line.strip_prefix("data: ").unwrap_or(line);
            if line == "[DONE]" || line.is_empty() {
                continue;
            }
            if let Ok(chunk) = serde_json::from_str::<StreamChunk>(line) {
                if let Some(content) = chunk.choices.first().and_then(|c| c.delta.content.as_ref()) {
                    chunks.push(content.clone());
                }
            }
        }
        Ok(chunks)
    }
}
