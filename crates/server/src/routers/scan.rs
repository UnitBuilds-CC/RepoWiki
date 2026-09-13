use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::stream::Stream;
use serde_json::json;
use uuid::Uuid;

use repowiki_analyzer::Analyzer;
use repowiki_core::config::{resolve_model, Config};
use repowiki_graph::DependencyGraph;
use repowiki_ingest::{ingest_github, ingest_local};
use repowiki_llm::client::LLMClient;
use repowiki_wiki::WikiBuilder;

use crate::models::{ProjectInfo, ScanRequest};
use crate::{AppState, ProjectState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/scan", post(start_scan))
        .route("/project/{project_id}", get(get_project))
        .route("/project/{project_id}/status", get(stream_status))
}

async fn start_scan(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ScanRequest>,
) -> Json<ProjectInfo> {
    let project_id = Uuid::new_v4().to_string()[..8].to_string();
    let info = ProjectInfo::new(project_id.clone());

    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    {
        let mut projects = state.projects.lock().await;
        projects.insert(
            project_id.clone(),
            ProjectState {
                info: info.clone(),
                wiki: None,
                project: None,
                rag: None,
                progress: Vec::new(),
            },
        );
    }

    let state_clone = state.clone();
    let pid = project_id.clone();
    tokio::spawn(async move {
        run_scan(state_clone, pid, req, api_key).await;
    });

    Json(info)
}

async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let projects = state.projects.lock().await;
    match projects.get(&project_id) {
        Some(proj) => Json(json!(proj.info)).into_response(),
        None => Json(json!({"error": "Project not found"})).into_response(),
    }
}

async fn stream_status(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let projects = state.projects.clone();
    let pid = project_id.clone();

    let stream = async_stream::stream! {
        let mut seen = 0usize;
        loop {
            let projects = projects.lock().await;
            let proj = match projects.get(&pid) {
                Some(p) => p,
                None => {
                    yield Ok(Event::default().data(json!({"error": "not found"}).to_string()));
                    return;
                }
            };

            while seen < proj.progress.len() {
                let step = &proj.progress[seen];
                yield Ok(Event::default().data(json!({"step": step}).to_string()));
                seen += 1;
            }

            let status = proj.info.status.clone();
            drop(projects);

            if status == "done" || status == "error" {
                yield Ok(Event::default().data(json!({"status": status}).to_string()));
                return;
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    };

    Sse::new(stream)
}

async fn run_scan(state: Arc<AppState>, project_id: String, req: ScanRequest, user_api_key: Option<String>) {
    let result = run_scan_inner(&state, &project_id, &req, user_api_key).await;

    if let Err(e) = result {
        let mut projects = state.projects.lock().await;
        if let Some(proj) = projects.get_mut(&project_id) {
            proj.info.status = "error".to_string();
            proj.info.error = e.clone();
            proj.progress.push(format!("Error: {}", e));
        }
    }
}

async fn run_scan_inner(
    state: &AppState,
    project_id: &str,
    req: &ScanRequest,
    user_api_key: Option<String>,
) -> Result<(), String> {
    let mut cfg = Config::load();
    if !req.language.is_empty() {
        cfg.language = req.language.clone();
    }
    if let Some(ref model) = req.model {
        cfg.model = resolve_model(model);
    }
    if let Some(ref key) = user_api_key {
        cfg.api_key = key.clone();
    } else if let Some(ref key) = req.api_key {
        cfg.api_key = key.clone();
    }

    {
        let mut projects = state.projects.lock().await;
        if let Some(proj) = projects.get_mut(project_id) {
            proj.progress.push("Ingesting project...".to_string());
        }
    }

    let project = if let Some(ref url) = req.url {
        ingest_github(url, cfg.max_file_size, cfg.max_files, false, &[])
            .map_err(|e| e.to_string())?
    } else if let Some(ref path) = req.path {
        ingest_local(path, cfg.max_file_size, cfg.max_files, &[])
            .map_err(|e| e.to_string())?
    } else {
        return Err("Either path or url must be provided".to_string());
    };

    {
        let mut projects = state.projects.lock().await;
        if let Some(proj) = projects.get_mut(project_id) {
            proj.info.status = "scanning".to_string();
            proj.info.name = project.name.clone();
            proj.info.total_files = project.files.len();
            proj.info.total_lines = project.files.iter().map(|f| f.lines as u64).sum();
            proj.project = Some(project.clone());
        }
    }

    if cfg.api_key.is_empty() {
        return Err("No API key configured".to_string());
    }

    {
        let mut projects = state.projects.lock().await;
        if let Some(proj) = projects.get_mut(project_id) {
            proj.progress.push("Analyzing codebase...".to_string());
        }
    }

    let llm = LLMClient::new(&cfg.model, &cfg.api_key, &cfg.api_base);
    let mut analyzer = Analyzer::new(llm, state.cache.clone(), &cfg.language, cfg.concurrency as usize);

    let projects_arc = state.projects.clone();
    let pid = project_id.to_string();
    let progress_fn = Arc::new(move |msg: &str| {
        let projects = projects_arc.clone();
        let pid = pid.clone();
        let m = msg.to_string();
        tokio::spawn(async move {
            let mut projects = projects.lock().await;
            if let Some(proj) = projects.get_mut(&pid) {
                proj.progress.push(m);
            }
        });
    });

    let wiki_data = analyzer.analyze(&project, Some(progress_fn)).await;

    let graph = DependencyGraph::build_from_project(&project);
    let builder = WikiBuilder;
    let wiki = builder.build(&project, &wiki_data, &graph);

    {
        let mut projects = state.projects.lock().await;
        if let Some(proj) = projects.get_mut(project_id) {
            proj.wiki = Some(wiki);
            proj.info.status = "done".to_string();
            proj.progress.push("Done!".to_string());
        }
    }

    Ok(())
}
