pub mod models;
pub mod routers;

use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use repowiki_cache::Cache;
use repowiki_core::models::ProjectContext;
use repowiki_rag::SimpleRAG;
use repowiki_wiki::Wiki;

use models::ProjectInfo;

pub struct ProjectState {
    pub info: ProjectInfo,
    pub wiki: Option<Wiki>,
    pub project: Option<ProjectContext>,
    pub rag: Option<Arc<SimpleRAG>>,
    pub progress: Vec<String>,
}

pub struct AppState {
    pub cache: Cache,
    pub projects: Arc<Mutex<HashMap<String, ProjectState>>>,
}

impl AppState {
    pub fn new(cache: Cache) -> Self {
        Self {
            cache,
            projects: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            [
                "http://localhost:5173".parse().unwrap(),
                "http://localhost:3000".parse().unwrap(),
                "http://127.0.0.1:5173".parse().unwrap(),
            ]
            .to_vec(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .merge(routers::scan::routes())
        .merge(routers::wiki::routes())
        .merge(routers::chat::routes());

    Router::new()
        .nest("/api", api)
        .layer(cors)
        .with_state(Arc::new(state))
}
