use std::collections::{HashMap, HashSet};
use std::path::Path;

use repowiki_cache::{content_hash, Cache};
use repowiki_core::models::*;

mod extract;
mod flow;
mod resolve;

pub use flow::trace_flow;

pub fn build_index(files: &[FileInfo], cache: &Cache, key_prefix: &str) -> ProjectIndex {
    let path_set: HashSet<&str> = files.iter().map(|f| f.path.as_str()).collect();

    let mut indexed_files: Vec<IndexedFile> = Vec::new();

    for f in files {
        let content = if !f.content.is_empty() {
            &f.content
        } else {
            &f.preview
        };
        if content.is_empty() {
            continue;
        }

        let hash = content_hash(content);
        let cache_key = format!("{}:index:{}:{}", key_prefix, f.path, hash);

        let indexed = if let Some(val) = cache.get_default_ttl(&cache_key) {
            match serde_json::from_value::<IndexedFile>(val) {
                Ok(mut idx) => {
                    idx.content_hash = hash.clone();
                    idx
                }
                Err(_) => index_one_file(f, content, &hash, &path_set),
            }
        } else {
            let idx = index_one_file(f, content, &hash, &path_set);
            if let Ok(val) = serde_json::to_value(&idx) {
                let _ = cache.put(&cache_key, &val);
            }
            idx
        };

        indexed_files.push(indexed);
    }

    let symbol_index = build_symbol_index(&indexed_files);
    let call_graph = build_call_graph(&indexed_files, &symbol_index);
    let modules = group_into_module_indices(indexed_files);

    ProjectIndex {
        modules,
        call_graph,
        symbol_index,
    }
}

fn index_one_file(
    file: &FileInfo,
    content: &str,
    hash: &str,
    path_set: &HashSet<&str>,
) -> IndexedFile {
    let (symbols, mut imports) = extract::extract_symbols(&file.path, content, &file.language);
    resolve::resolve_imports(&mut imports, &file.path, &file.language, path_set);
    let metrics = extract::compute_metrics(content, &file.language);

    let exports: Vec<String> = symbols
        .iter()
        .filter(|s| s.visibility == Visibility::Public)
        .map(|s| s.name.clone())
        .collect();

    IndexedFile {
        path: file.path.clone(),
        language: file.language.clone(),
        size: file.size as usize,
        lines: file.lines as usize,
        metrics,
        symbols,
        imports,
        exports,
        content_hash: hash.to_string(),
    }
}

fn build_symbol_index(files: &[IndexedFile]) -> HashMap<String, Vec<String>> {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for f in files {
        for sym in &f.symbols {
            index
                .entry(sym.name.clone())
                .or_default()
                .push(f.path.clone());
        }
    }
    index
}

fn build_call_graph(
    files: &[IndexedFile],
    symbol_index: &HashMap<String, Vec<String>>,
) -> Vec<CallEdge> {
    let mut edges = Vec::new();
    let mut seen: HashSet<(String, String, String, String)> = HashSet::new();

    for f in files {
        for sym in &f.symbols {
            for call in &sym.calls {
                if let Some(target_files) = symbol_index.get(call) {
                    for target_file in target_files {
                        if target_file == &f.path {
                            continue;
                        }
                        let key = (
                            f.path.clone(),
                            sym.name.clone(),
                            target_file.clone(),
                            call.clone(),
                        );
                        if seen.insert(key) {
                            edges.push(CallEdge {
                                from_file: f.path.clone(),
                                from_symbol: sym.name.clone(),
                                to_file: target_file.clone(),
                                to_symbol: call.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    edges
}

fn group_into_module_indices(files: Vec<IndexedFile>) -> Vec<ModuleIndex> {
    let mut groups: HashMap<String, Vec<IndexedFile>> = HashMap::new();

    for f in files {
        let mod_name = module_name_for(&f.path);
        groups.entry(mod_name).or_default().push(f);
    }

    let file_paths: HashSet<String> = groups
        .values()
        .flatten()
        .map(|f| f.path.clone())
        .collect();

    let mut modules: Vec<ModuleIndex> = Vec::new();
    for (name, mod_files) in groups {
        let mut internal_edges = Vec::new();
        let mut external_deps: HashSet<String> = HashSet::new();

        for f in &mod_files {
            for imp in &f.imports {
                if let Some(ref resolved) = imp.resolved_path {
                    if file_paths.contains(resolved) {
                        let target_mod = module_name_for(resolved);
                        if target_mod == name {
                            internal_edges.push((f.path.clone(), resolved.clone()));
                        } else {
                            external_deps.insert(target_mod);
                        }
                    }
                }
            }
        }

        let total_symbols = mod_files.iter().map(|f| f.symbols.len()).sum();
        let total_complexity = mod_files.iter().map(|f| f.metrics.complexity).sum();

        let ext_deps: Vec<(String, String)> = external_deps
            .into_iter()
            .map(|d| (name.clone(), d))
            .collect();

        modules.push(ModuleIndex {
            name,
            files: mod_files,
            internal_edges,
            external_deps: ext_deps,
            total_symbols,
            total_complexity,
        });
    }

    modules.sort_by(|a, b| {
        b.files
            .len()
            .cmp(&a.files.len())
            .then_with(|| a.name.cmp(&b.name))
    });

    modules
}

fn module_name_for(path: &str) -> String {
    let p = Path::new(path);
    let parts: Vec<&str> = p
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() <= 1 {
        "root".to_string()
    } else {
        let first = parts[0];
        if matches!(first, "src" | "lib" | "pkg" | "internal" | "app" | "crates") && parts.len() > 2 {
            parts[1].to_string()
        } else {
            first.to_string()
        }
    }
}

pub fn format_module_context(module: &ModuleIndex) -> String {
    let mut parts = Vec::new();

    parts.push(format!(
        "## Module Stats\n- {} files, {} symbols, {} cyclomatic complexity",
        module.files.len(),
        module.total_symbols,
        module.total_complexity
    ));

    parts.push("\n## Files".to_string());
    for f in &module.files {
        let mut file_section = format!(
            "### {} ({}, {} lines, {} code, {} complexity)",
            f.path, f.language, f.lines, f.metrics.code_lines, f.metrics.complexity
        );

        if !f.imports.is_empty() {
            let import_strs: Vec<String> = f
                .imports
                .iter()
                .map(|imp| {
                    if let Some(ref resolved) = imp.resolved_path {
                        format!("{} -> {}", imp.raw, resolved)
                    } else {
                        imp.raw.clone()
                    }
                })
                .collect();
            file_section.push_str(&format!("\nImports: {}", import_strs.join(", ")));
        }

        if !f.symbols.is_empty() {
            file_section.push_str("\nSymbols:");
            for sym in &f.symbols {
                let vis = match sym.visibility {
                    Visibility::Public => "pub ",
                    Visibility::Protected => "protected ",
                    Visibility::Private => "",
                };

                let params_str = if sym.params.is_empty() {
                    String::new()
                } else {
                    let ps: Vec<String> = sym
                        .params
                        .iter()
                        .map(|p| {
                            if let Some(ref t) = p.type_hint {
                                format!("{}: {}", p.name, t)
                            } else {
                                p.name.clone()
                            }
                        })
                        .collect();
                    format!("({})", ps.join(", "))
                };

                let ret_str = sym
                    .return_type
                    .as_ref()
                    .map(|r| format!(" -> {}", r))
                    .unwrap_or_default();

                let mut line = format!("  - {}{} {}{}  [line {}]", vis, sym.kind, sym.name, params_str, sym.line);
                if !ret_str.is_empty() {
                    line = format!("  - {}{} {}{}{}  [line {}]", vis, sym.kind, sym.name, params_str, ret_str, sym.line);
                }

                if !sym.calls.is_empty() {
                    line.push_str(&format!("\n    calls: [{}]", sym.calls.join(", ")));
                }

                if let Some(ref doc) = sym.doc_comment {
                    let short = if doc.len() > 120 {
                        format!("{}...", &doc[..120])
                    } else {
                        doc.clone()
                    };
                    line.push_str(&format!("\n    doc: {}", short));
                }

                if !sym.fields.is_empty() {
                    line.push_str(&format!("\n    fields: [{}]", sym.fields.join(", ")));
                }

                file_section.push_str(&format!("\n{}", line));
            }
        }

        parts.push(file_section);
    }

    if !module.internal_edges.is_empty() {
        parts.push("\n## Internal References".to_string());
        for (src, dst) in &module.internal_edges {
            parts.push(format!("- {} -> {}", src, dst));
        }
    }

    if !module.external_deps.is_empty() {
        parts.push("\n## External Dependencies".to_string());
        for (src, dst) in &module.external_deps {
            parts.push(format!("- {} depends on {}", src, dst));
        }
    }

    parts.join("\n")
}
