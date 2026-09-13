use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use repowiki_graph::DependencyGraph;
use repowiki_wiki::SidebarItem;

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/project/{project_id}/wiki", get(get_wiki))
        .route("/project/{project_id}/wiki/{*page_id}", get(get_page))
        .route("/project/{project_id}/file/{*file_path}", get(get_file))
        .route("/project/{project_id}/graph", get(get_graph))
}

async fn get_wiki(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let projects = state.projects.lock().await;
    let proj = match projects.get(&project_id) {
        Some(p) => p,
        None => return Json(json!({"error": "Wiki not ready"})).into_response(),
    };

    let wiki = match &proj.wiki {
        Some(w) => w,
        None => return Json(json!({"error": "Wiki not ready"})).into_response(),
    };

    let sidebar = serialize_sidebar(&wiki.sidebar);
    let pages: Vec<_> = wiki
        .pages
        .iter()
        .map(|p| {
            json!({
                "id": p.id,
                "title": p.title,
                "order": p.order,
                "parent_id": p.parent_id,
            })
        })
        .collect();

    Json(json!({
        "project_name": wiki.project_name,
        "sidebar": sidebar,
        "pages": pages,
    }))
    .into_response()
}

async fn get_page(
    State(state): State<Arc<AppState>>,
    Path((project_id, page_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let projects = state.projects.lock().await;
    let proj = match projects.get(&project_id) {
        Some(p) => p,
        None => return Json(json!({"error": "Wiki not ready"})).into_response(),
    };

    let wiki = match &proj.wiki {
        Some(w) => w,
        None => return Json(json!({"error": "Wiki not ready"})).into_response(),
    };

    match wiki.get_page(&page_id) {
        Some(page) => Json(json!({
            "id": page.id,
            "title": page.title,
            "content": page.content,
        }))
        .into_response(),
        None => Json(json!({"error": format!("Page '{}' not found", page_id)})).into_response(),
    }
}

async fn get_file(
    State(state): State<Arc<AppState>>,
    Path((project_id, file_path)): Path<(String, String)>,
) -> impl IntoResponse {
    let projects = state.projects.lock().await;
    let proj = match projects.get(&project_id) {
        Some(p) => p,
        None => return Json(json!({"error": "Project not ready"})).into_response(),
    };

    let project = match &proj.project {
        Some(p) => p,
        None => return Json(json!({"error": "Project not ready"})).into_response(),
    };

    for f in &project.files {
        if f.path == file_path {
            let content = if !f.content.is_empty() {
                &f.content
            } else {
                &f.preview
            };
            return Json(json!({
                "path": f.path,
                "language": f.language,
                "content": content,
                "lines": f.lines,
            }))
            .into_response();
        }
    }

    Json(json!({"error": format!("File '{}' not found", file_path)})).into_response()
}

async fn get_graph(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let projects = state.projects.lock().await;
    let proj = match projects.get(&project_id) {
        Some(p) => p,
        None => return Json(json!({"error": "Project not ready"})).into_response(),
    };

    let project = match &proj.project {
        Some(p) => p,
        None => return Json(json!({"error": "Project not ready"})).into_response(),
    };

    let graph = DependencyGraph::build_from_project(project);

    let nodes: Vec<_> = graph.nodes().iter().map(|n| json!({"id": n})).collect();
    let edges: Vec<_> = graph
        .edges()
        .iter()
        .map(|(s, t)| json!({"source": s, "target": t}))
        .collect();
    let rankings: Vec<_> = graph
        .rank_files()
        .iter()
        .take(20)
        .map(|(path, score)| json!({"path": path, "score": (score * 1e6).round() / 1e6}))
        .collect();

    Json(json!({
        "nodes": nodes,
        "edges": edges,
        "rankings": rankings,
        "mermaid": graph.to_mermaid(),
    }))
    .into_response()
}

fn serialize_sidebar(items: &[SidebarItem]) -> Vec<serde_json::Value> {
    items
        .iter()
        .map(|item| {
            let mut entry = json!({
                "title": item.title,
                "page_id": item.page_id,
            });
            if !item.children.is_empty() {
                entry["children"] = json!(serialize_sidebar(&item.children));
            }
            entry
        })
        .collect()
}
