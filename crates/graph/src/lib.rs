use std::collections::{HashMap, HashSet};

use petgraph::algo::tarjan_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use regex::Regex;
use repowiki_core::models::ProjectContext;

pub struct DependencyGraph {
    graph: DiGraph<String, ()>,
}

impl DependencyGraph {
    pub fn build_from_project(project: &ProjectContext) -> Self {
        let mut graph = DiGraph::new();
        let mut node_idx: HashMap<String, NodeIndex> = HashMap::new();

        for f in &project.files {
            let idx = graph.add_node(f.path.clone());
            node_idx.insert(f.path.clone(), idx);
        }

        let path_set: HashSet<&str> = project.files.iter().map(|f| f.path.as_str()).collect();

        for f in &project.files {
            let content = if !f.content.is_empty() {
                &f.content
            } else {
                &f.preview
            };
            if content.is_empty() {
                continue;
            }

            let patterns = import_patterns(&f.language);
            for pat in patterns {
                for cap in pat.captures_iter(content) {
                    let import_path = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    if let Some(resolved) =
                        resolve_import(import_path, &f.path, &f.language, &path_set)
                    {
                        if resolved != f.path {
                            if let (Some(&src), Some(&dst)) =
                                (node_idx.get(&f.path), node_idx.get(&resolved))
                            {
                                graph.add_edge(src, dst, ());
                            }
                        }
                    }
                }
            }
        }

        Self { graph }
    }

    pub fn rank_files(&self) -> Vec<(String, f64)> {
        if self.graph.node_count() == 0 {
            return Vec::new();
        }
        let scores = pagerank_power_iteration(&self.graph, 0.85, 100, 1.0e-6);
        let mut result: Vec<(String, f64)> = self
            .graph
            .node_indices()
            .map(|i| (self.graph[i].clone(), scores[&i]))
            .collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        result
    }

    pub fn get_core_files(&self, top_n: usize) -> Vec<String> {
        self.rank_files()
            .into_iter()
            .take(top_n)
            .map(|(p, _)| p)
            .collect()
    }

    pub fn get_module_dependencies(&self) -> HashMap<String, HashSet<String>> {
        let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
        for edge in self.graph.edge_indices() {
            let (src, dst) = self.graph.edge_endpoints(edge).unwrap();
            let src_mod = get_module(&self.graph[src]);
            let dst_mod = get_module(&self.graph[dst]);
            if src_mod != dst_mod {
                deps.entry(src_mod).or_default().insert(dst_mod);
            }
        }
        deps
    }

    pub fn to_mermaid(&self) -> String {
        let mod_deps = self.get_module_dependencies();
        if mod_deps.is_empty() {
            return String::new();
        }

        let mut lines = vec!["graph TD".to_string()];
        let mut seen_edges: HashSet<(String, String)> = HashSet::new();
        let mut src_keys: Vec<&String> = mod_deps.keys().collect();
        src_keys.sort();
        for src in src_keys {
            let targets = &mod_deps[src];
            let mut dst_keys: Vec<&String> = targets.iter().collect();
            dst_keys.sort();
            for dst in dst_keys {
                let edge = (src.clone(), dst.clone());
                if seen_edges.insert(edge) {
                    let s = mermaid_id(src);
                    let d = mermaid_id(dst);
                    lines.push(format!("  {s}[{src}] --> {d}[{dst}]"));
                }
            }
        }
        lines.join("\n")
    }

    pub fn get_entry_points(&self) -> Vec<String> {
        let mut entries = Vec::new();
        for node in self.graph.node_indices() {
            let in_deg = self.graph.edges_directed(node, petgraph::Direction::Incoming).count();
            let out_deg = self.graph.edges_directed(node, petgraph::Direction::Outgoing).count();
            if in_deg <= 1 && out_deg > 0 {
                entries.push(self.graph[node].clone());
            }
        }
        entries
    }

    pub fn find_isolated_files(&self) -> Vec<String> {
        let mut isolated = Vec::new();
        for node in self.graph.node_indices() {
            let in_deg = self.graph.edges_directed(node, petgraph::Direction::Incoming).count();
            let out_deg = self.graph.edges_directed(node, petgraph::Direction::Outgoing).count();
            if in_deg == 0 && out_deg == 0 {
                isolated.push(self.graph[node].clone());
            }
        }
        isolated.sort();
        isolated
    }

    pub fn find_circular_dependencies(&self, limit: usize) -> Vec<Vec<String>> {
        let sccs = tarjan_scc(&self.graph);
        let mut cycles: Vec<Vec<String>> = sccs
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| {
                let mut paths: Vec<String> = scc.iter().map(|&i| self.graph[i].clone()).collect();
                paths.sort();
                paths
            })
            .collect();
        cycles.sort_by(|a, b| b.len().cmp(&a.len()).then(a[0].cmp(&b[0])));
        cycles.truncate(limit);
        cycles
    }

    pub fn nodes(&self) -> Vec<String> {
        self.graph.node_indices().map(|i| self.graph[i].clone()).collect()
    }

    pub fn edges(&self) -> Vec<(String, String)> {
        self.graph
            .edge_indices()
            .filter_map(|e| {
                let (src, dst) = self.graph.edge_endpoints(e)?;
                Some((self.graph[src].clone(), self.graph[dst].clone()))
            })
            .collect()
    }
}

fn pagerank_power_iteration(
    graph: &DiGraph<String, ()>,
    alpha: f64,
    max_iter: usize,
    tol: f64,
) -> HashMap<NodeIndex, f64> {
    let n = graph.node_count();
    if n == 0 {
        return HashMap::new();
    }
    let nodes: Vec<NodeIndex> = graph.node_indices().collect();
    let nf = n as f64;

    let out_degree: HashMap<NodeIndex, usize> = nodes
        .iter()
        .map(|&node| {
            let deg = graph.edges_directed(node, petgraph::Direction::Outgoing).count();
            (node, deg)
        })
        .collect();

    let mut scores: HashMap<NodeIndex, f64> = nodes.iter().map(|&n| (n, 1.0 / nf)).collect();

    for _ in 0..max_iter {
        let dangling: f64 = nodes
            .iter()
            .filter(|&&node| out_degree[&node] == 0)
            .map(|&node| scores[&node])
            .sum();

        let mut new_scores: HashMap<NodeIndex, f64> = HashMap::new();
        for &node in &nodes {
            let incoming: f64 = graph
                .edges_directed(node, petgraph::Direction::Incoming)
                .filter_map(|e| {
                    let pred = e.source();
                    if out_degree[&pred] > 0 {
                        Some(scores[&pred] / out_degree[&pred] as f64)
                    } else {
                        None
                    }
                })
                .sum();
            new_scores.insert(node, (1.0 - alpha) / nf + alpha * (incoming + dangling / nf));
        }

        let diff: f64 = nodes
            .iter()
            .map(|&node| (new_scores[&node] - scores[&node]).abs())
            .sum();

        scores = new_scores;
        if diff < tol {
            break;
        }
    }
    scores
}

fn get_module(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() <= 1 {
        return "root".into();
    }
    let mod_name = parts[0];
    if matches!(mod_name, "src" | "lib" | "pkg" | "internal" | "app") && parts.len() > 2 {
        return parts[1].to_string();
    }
    mod_name.to_string()
}

fn mermaid_id(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

fn import_patterns(language: &str) -> Vec<Regex> {
    let base = match language {
        "python" | "pyi" => vec![
            Regex::new(r"(?m)^\s*import\s+([\w.]+)").unwrap(),
            Regex::new(r"(?m)^\s*from\s+([\w.]+)\s+import").unwrap(),
        ],
        "javascript" | "jsx" | "mjs" | "cjs" => vec![
            Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap(),
            Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap(),
        ],
        "typescript" | "tsx" | "mts" => vec![
            Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap(),
            Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap(),
        ],
        "go" => vec![Regex::new(r#"(?m)"([^"]+)""#).unwrap()],
        "rust" => vec![
            Regex::new(r"(?m)^\s*use\s+([\w:]+)").unwrap(),
            Regex::new(r"(?m)^\s*mod\s+(\w+)").unwrap(),
        ],
        "java" => vec![Regex::new(r"(?m)^\s*import\s+([\w.]+);").unwrap()],
        _ => vec![],
    };
    base
}

fn resolve_import(
    import_path: &str,
    source_file: &str,
    language: &str,
    known_paths: &HashSet<&str>,
) -> Option<String> {
    let candidates: Vec<String> = match language {
        "python" | "pyi" => {
            let rel = resolve_python_module(import_path, source_file);
            let mut c = vec![
                format!("{rel}.py"),
                format!("{rel}/__init__.py"),
            ];
            if !rel.starts_with("src/") {
                c.push(format!("src/{rel}.py"));
                c.push(format!("src/{rel}/__init__.py"));
            }
            c
        }
        "javascript" | "typescript" | "jsx" | "tsx" | "mjs" | "cjs" | "mts" => {
            let rel = if import_path.starts_with('.') {
                let base_dir = source_file.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
                normalize_path(&format!("{base_dir}/{import_path}"))
            } else {
                import_path.to_string()
            };
            vec![
                rel.clone(),
                format!("{rel}.ts"),
                format!("{rel}.tsx"),
                format!("{rel}.js"),
                format!("{rel}.jsx"),
                format!("{rel}.mjs"),
                format!("{rel}.cjs"),
                format!("{rel}/index.ts"),
                format!("{rel}/index.tsx"),
                format!("{rel}/index.js"),
                format!("{rel}/index.jsx"),
                format!("{rel}/index.mjs"),
                format!("{rel}/index.cjs"),
            ]
        }
        "go" => {
            let parts: Vec<&str> = import_path.split('/').collect();
            if parts.len() >= 2 {
                vec![format!("{}/{}.go", parts[parts.len() - 2], parts[parts.len() - 1])]
            } else {
                return None;
            }
        }
        "rust" => {
            let rel = import_path.split("::").next().unwrap_or("").replace("::", "/");
            vec![
                format!("src/{rel}.rs"),
                format!("src/{rel}/mod.rs"),
                format!("{rel}.rs"),
            ]
        }
        "java" => {
            let rel = import_path.replace('.', "/");
            vec![
                format!("src/main/java/{rel}.java"),
                format!("{rel}.java"),
            ]
        }
        _ => return None,
    };

    for c in candidates {
        let normalized = normalize_path(&c.replace('\\', "/"));
        if known_paths.contains(normalized.as_str()) {
            return Some(normalized);
        }
    }
    None
}

fn resolve_python_module(import_path: &str, source_file: &str) -> String {
    let leading_dots = import_path.chars().take_while(|&c| c == '.').count();
    let module = import_path[leading_dots..].replace('.', "/");
    if leading_dots == 0 {
        return module;
    }

    let source_normalized = source_file.replace('\\', "/");
    let source_dir: Vec<&str> = source_normalized
        .rsplit_once('/')
        .map(|(d, _)| d.split('/').collect())
        .unwrap_or_default();
    let keep = (source_dir.len() as i32 - leading_dots as i32 + 1).max(0) as usize;
    let mut parts: Vec<String> = source_dir[..keep].iter().map(|s| s.to_string()).collect();
    if !module.is_empty() {
        parts.extend(module.split('/').map(|s| s.to_string()));
    }
    parts.join("/")
}

fn normalize_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}
