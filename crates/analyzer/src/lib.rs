use std::collections::HashMap;
use std::sync::Arc;

use repowiki_cache::{content_hash, Cache};
use repowiki_core::models::*;
use repowiki_graph::DependencyGraph;
use repowiki_index::{build_index, format_module_context};
use repowiki_llm::client::LLMClient;
use repowiki_llm::prompts::{
    build_architecture_prompt, build_module_prompt_from_index, build_overview_prompt,
    build_reading_guide_prompt, extract_json,
};
use tokio::sync::Semaphore;
use tracing::warn;

type ProgressFn = Arc<dyn Fn(&str) + Send + Sync>;

pub struct Analyzer {
    llm: LLMClient,
    cache: Cache,
    language: String,
    key_prefix: String,
    sem: Arc<Semaphore>,
    pub cache_keys: HashMap<String, String>,
}

impl Analyzer {
    pub fn new(llm: LLMClient, cache: Cache, language: &str, concurrency: usize) -> Self {
        let model = llm.model().to_string();
        Self {
            llm,
            cache,
            language: language.to_string(),
            key_prefix: format!("{model}:{language}"),
            sem: Arc::new(Semaphore::new(concurrency)),
            cache_keys: HashMap::new(),
        }
    }

    pub async fn analyze(
        &mut self,
        project: &ProjectContext,
        on_progress: Option<ProgressFn>,
    ) -> WikiData {
        self.cache_keys.clear();

        let progress = |msg: &str| {
            if let Some(ref cb) = on_progress {
                cb(msg);
            }
        };

        progress("Building project index...");
        let index = build_index(&project.files, &self.cache, &self.key_prefix);

        progress("Preparing file context...");
        let key_files_text = build_key_files_context(project);
        let tree_hash = content_hash(&(project.file_tree.clone() + &key_files_text));

        progress("Generating project overview...");
        let overview = self
            .generate_overview(project, &key_files_text, &tree_hash)
            .await;

        progress(&format!("Analyzing {} modules...", index.modules.len()));
        let module_docs = self
            .analyze_modules(&index, &overview.one_liner, &on_progress)
            .await;

        progress("Detecting architecture...");
        let module_summary = build_module_summary(&index);
        let arch_hash = content_hash(&(tree_hash.clone() + &module_summary));
        let architecture = self
            .generate_architecture(project, &key_files_text, &module_summary, &arch_hash)
            .await;

        progress("Creating reading guide...");
        let reading_guide = self
            .generate_reading_guide(project, &module_docs, &tree_hash)
            .await;

        progress("Done!");
        WikiData {
            overview,
            modules: module_docs,
            architecture,
            reading_guide,
            file_index: Default::default(),
        }
    }

    async fn generate_overview(
        &mut self,
        project: &ProjectContext,
        key_files: &str,
        tree_hash: &str,
    ) -> ProjectOverview {
        let cache_key = format!("{}:overview:{}", self.key_prefix, tree_hash);
        self.cache_keys.insert("index".into(), cache_key.clone());

        if let Some(val) = self.cache.get_default_ttl(&cache_key) {
            if let Ok(o) = serde_json::from_value::<ProjectOverview>(val) {
                return o;
            }
        }

        let messages =
            build_overview_prompt(&project.file_tree, key_files, &self.language);
        let raw = self.llm.complete(&messages, 0.3, 4096).await.unwrap_or_default();
        let overview = match extract_json(&raw) {
            Some(data) => serde_json::from_value(data).unwrap_or_else(|_| ProjectOverview {
                name: project.name.clone(),
                ..Default::default()
            }),
            None => {
                warn!("Failed to parse overview JSON, using defaults");
                ProjectOverview {
                    name: project.name.clone(),
                    ..Default::default()
                }
            }
        };

        if let Ok(val) = serde_json::to_value(&overview) {
            let _ = self.cache.put(&cache_key, &val);
        }
        overview
    }

    async fn analyze_modules(
        &mut self,
        index: &ProjectIndex,
        project_summary: &str,
        on_progress: &Option<ProgressFn>,
    ) -> Vec<ModuleDoc> {
        let mut handles = Vec::new();

        for module in &index.modules {
            let llm = self.llm.clone();
            let cache = self.cache.clone();
            let key_prefix = self.key_prefix.clone();
            let language = self.language.clone();
            let sem = self.sem.clone();
            let module = module.clone();
            let project_summary = project_summary.to_string();

            handles.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                analyze_one_module(
                    &llm, &cache, &key_prefix, &language, &module, &project_summary,
                )
                .await
            }));
        }

        let mut results = Vec::new();
        let mut reused = 0u32;
        let mut regenerated = 0u32;
        let total = handles.len();

        for (i, handle) in handles.into_iter().enumerate() {
            let (doc, was_cached) = handle.await.unwrap_or_default();
            if let Some(doc) = doc {
                results.push(doc);
                if was_cached {
                    reused += 1;
                } else {
                    regenerated += 1;
                }
            }
            if let Some(ref cb) = on_progress {
                cb(&format!("Analyzed module {}/{}", i + 1, total));
            }
        }

        if reused > 0 {
            if let Some(ref cb) = on_progress {
                cb(&format!(
                    "Module analysis: {} reused from cache, {} regenerated",
                    reused, regenerated
                ));
            }
        }

        results.sort_by(|a, b| {
            b.files
                .len()
                .cmp(&a.files.len())
                .then_with(|| a.name.cmp(&b.name))
        });
        results
    }

    async fn generate_architecture(
        &mut self,
        project: &ProjectContext,
        key_files: &str,
        module_summary: &str,
        tree_hash: &str,
    ) -> ArchitectureDiagram {
        let cache_key = format!("{}:arch:{}", self.key_prefix, tree_hash);
        self.cache_keys
            .insert("architecture".into(), cache_key.clone());

        if let Some(val) = self.cache.get_default_ttl(&cache_key) {
            if let Ok(a) = serde_json::from_value::<ArchitectureDiagram>(val) {
                return a;
            }
        }

        let messages =
            build_architecture_prompt(&project.file_tree, key_files, module_summary, &self.language);
        let raw = self.llm.complete(&messages, 0.3, 4096).await.unwrap_or_default();
        let arch = match extract_json(&raw) {
            Some(data) => serde_json::from_value(data).unwrap_or_default(),
            None => {
                warn!("Failed to parse architecture JSON");
                ArchitectureDiagram::default()
            }
        };

        if let Ok(val) = serde_json::to_value(&arch) {
            let _ = self.cache.put(&cache_key, &val);
        }
        arch
    }

    async fn generate_reading_guide(
        &mut self,
        project: &ProjectContext,
        module_docs: &[ModuleDoc],
        tree_hash: &str,
    ) -> ReadingGuide {
        let graph = DependencyGraph::build_from_project(project);
        let ranked = graph.rank_files();
        let by_path: HashMap<&str, &FileInfo> =
            project.files.iter().map(|f| (f.path.as_str(), f)).collect();

        let mut ranked_paths: Vec<String> = ranked
            .iter()
            .take(20)
            .map(|(path, _)| path.clone())
            .collect();
        let mut seen: std::collections::HashSet<String> =
            ranked_paths.iter().cloned().collect();
        for f in &project.files {
            if ranked_paths.len() >= 20 {
                break;
            }
            if seen.insert(f.path.clone()) {
                ranked_paths.push(f.path.clone());
            }
        }

        let mut rankings_parts = Vec::new();
        for (i, path) in ranked_paths.iter().enumerate() {
            if let Some(f) = by_path.get(path.as_str()) {
                let tag = if f.is_entrypoint {
                    " [entrypoint]"
                } else if f.is_config {
                    " [config]"
                } else {
                    ""
                };
                rankings_parts.push(format!("{}. {}{} ({} lines)", i + 1, path, tag, f.lines));
            }
        }
        let rankings = rankings_parts.join("\n");

        let module_parts: Vec<String> = module_docs
            .iter()
            .map(|m| format!("- **{}**: {}", m.name, m.purpose))
            .collect();
        let module_summaries = module_parts.join("\n");

        let cache_key = format!(
            "{}:guide:{}:{}",
            self.key_prefix,
            tree_hash,
            content_hash(&(rankings.clone() + &module_summaries))
        );
        self.cache_keys
            .insert("reading-guide".into(), cache_key.clone());

        if let Some(val) = self.cache.get_default_ttl(&cache_key) {
            if let Ok(g) = serde_json::from_value::<ReadingGuide>(val) {
                return g;
            }
        }

        let messages =
            build_reading_guide_prompt(&rankings, &module_summaries, &self.language);
        let raw = self.llm.complete(&messages, 0.3, 4096).await.unwrap_or_default();
        let guide = match extract_json(&raw) {
            Some(data) => serde_json::from_value(data).unwrap_or_default(),
            None => {
                warn!("Failed to parse reading guide JSON");
                ReadingGuide::default()
            }
        };

        if let Ok(val) = serde_json::to_value(&guide) {
            let _ = self.cache.put(&cache_key, &val);
        }
        guide
    }
}

fn build_key_files_context(project: &ProjectContext) -> String {
    let parts: Vec<String> = project
        .files
        .iter()
        .filter(|f| f.is_config || f.is_entrypoint)
        .map(|f| {
            let content = if !f.content.is_empty() {
                &f.content
            } else {
                &f.preview
            };
            let content = if content.len() > 4096 {
                format!("{}... (truncated)", &content[..4096])
            } else {
                content.clone()
            };
            format!("### {}\n```{}\n{}\n```", f.path, f.language, content)
        })
        .collect();
    parts.join("\n\n")
}

fn build_module_summary(index: &ProjectIndex) -> String {
    if index.modules.is_empty() {
        return String::new();
    }
    let mut parts = Vec::new();
    for m in &index.modules {
        let top_symbols: Vec<String> = m
            .files
            .iter()
            .flat_map(|f| f.symbols.iter())
            .take(5)
            .map(|s| s.name.clone())
            .collect();
        let syms = if top_symbols.is_empty() {
            String::new()
        } else {
            format!(" — {}", top_symbols.join(", "))
        };
        let file_paths: Vec<String> = m.files.iter().map(|f| f.path.clone()).collect();
        let files_str = if file_paths.is_empty() {
            String::new()
        } else {
            format!("\n  Files: {}", file_paths.join(", "))
        };
        parts.push(format!(
            "- {} ({} files, {} symbols){}{}",
            m.name,
            m.files.len(),
            m.total_symbols,
            syms,
            files_str
        ));
    }
    parts.join("\n")
}

async fn analyze_one_module(
    llm: &LLMClient,
    cache: &Cache,
    key_prefix: &str,
    language: &str,
    module: &ModuleIndex,
    project_summary: &str,
) -> (Option<ModuleDoc>, bool) {
    let index_context = format_module_context(module);

    let content_hashes: Vec<String> = module
        .files
        .iter()
        .map(|f| f.content_hash.clone())
        .collect();
    let cache_key = format!(
        "{}:module:{}:{}",
        key_prefix,
        module.name,
        content_hash(&content_hashes.join(":"))
    );

    if let Some(val) = cache.get_default_ttl(&cache_key) {
        if let Ok(doc) = serde_json::from_value::<ModuleDoc>(val) {
            return (Some(doc), true);
        }
    }

    let messages =
        build_module_prompt_from_index(&module.name, &index_context, project_summary, language);
    let raw = match llm.complete(&messages, 0.3, 4096).await {
        Ok(r) => r,
        Err(e) => {
            warn!("Module '{}' LLM error: {:?}", module.name, e);
            return (
                Some(ModuleDoc {
                    name: module.name.clone(),
                    purpose: format!("Module containing {} files", module.files.len()),
                    ..Default::default()
                }),
                false,
            );
        }
    };

    let doc = match extract_json(&raw) {
        Some(mut data) => {
            if let Some(obj) = data.as_object_mut() {
                obj.entry("name")
                    .or_insert_with(|| serde_json::Value::String(module.name.clone()));
            }
            match serde_json::from_value::<ModuleDoc>(data) {
                Ok(doc) => doc,
                Err(e) => {
                    warn!("Failed to parse module '{}' JSON: {:?}", module.name, e);
                    ModuleDoc {
                        name: module.name.clone(),
                        ..Default::default()
                    }
                }
            }
        }
        None => {
            warn!("Failed to parse module '{}' JSON", module.name);
            ModuleDoc {
                name: module.name.clone(),
                purpose: format!("Module containing {} files", module.files.len()),
                ..Default::default()
            }
        }
    };

    if let Ok(val) = serde_json::to_value(&doc) {
        let _ = cache.put(&cache_key, &val);
    }
    (Some(doc), false)
}
