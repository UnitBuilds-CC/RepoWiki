use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use repowiki_core::config::Config;
use repowiki_llm::client::{ChatMessage, LLMClient};
use repowiki_llm::prompts::build_chat_prompt;
use repowiki_rag::load_or_build_index;

use crate::models::{ChatRequest, FileReference};
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/project/{project_id}/chat", post(chat))
}

async fn chat(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(project_id): Path<String>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    let rag = {
        let mut projects = state.projects.lock().await;
        let proj = match projects.get_mut(&project_id) {
            Some(p) => p,
            None => return Json(json!({"error": "Project not ready"})).into_response(),
        };

        let project = match &proj.project {
            Some(p) => p.clone(),
            None => return Json(json!({"error": "Project not ready"})).into_response(),
        };

        if proj.rag.is_none() {
            let (rag, _) = load_or_build_index(&project, None);
            proj.rag = Some(Arc::new(rag));
        }
        proj.rag.clone().unwrap()
    };

    let chunks = rag.retrieve(&req.question, 5);
    let mut context_parts = Vec::new();
    let mut references = Vec::new();

    for chunk in &chunks {
        context_parts.push(format!(
            "### {} (lines {}-{})\n```\n{}\n```",
            chunk.file_path, chunk.line_start, chunk.line_end, chunk.content
        ));
        references.push(FileReference {
            path: chunk.file_path.clone(),
            line_start: chunk.line_start,
            line_end: chunk.line_end,
            snippet: chunk.content.chars().take(200).collect(),
        });
    }

    let context_text = context_parts.join("\n\n");

    let mut cfg = Config::load();
    if let Some(key) = headers.get("x-api-key").and_then(|v| v.to_str().ok()) {
        cfg.api_key = key.to_string();
    }

    if cfg.api_key.is_empty() {
        return Json(json!({"error": "No API key configured"})).into_response();
    }

    let llm = LLMClient::new(&cfg.model, &cfg.api_key, &cfg.api_base);
    let history: Vec<ChatMessage> = req
        .history
        .iter()
        .map(|t| ChatMessage {
            role: t.role.clone(),
            content: t.content.clone(),
        })
        .collect();
    let messages = build_chat_prompt(&req.question, &context_text, &cfg.language, &history);

    let refs_json = json!(references);
    let stream = async_stream::stream! {
        yield Ok::<Event, std::convert::Infallible>(Event::default().data(json!({"references": refs_json}).to_string()));

        match llm.stream(&messages, 0.3, 2048).await {
            Ok(chunks) => {
                for chunk in chunks {
                    yield Ok(Event::default().data(json!({"content": chunk}).to_string()));
                }
            }
            Err(e) => {
                yield Ok(Event::default().data(json!({"error": e.to_string()}).to_string()));
            }
        }

        yield Ok(Event::default().data(json!({"done": true}).to_string()));
    };

    Sse::new(stream).into_response()
}
