use std::collections::HashMap;

use regex::Regex;
use repowiki_core::models::*;
use repowiki_graph::DependencyGraph;

#[derive(Debug, Clone, Default)]
pub struct WikiPage {
    pub id: String,
    pub title: String,
    pub content: String,
    pub parent_id: String,
    pub order: i32,
}

#[derive(Debug, Clone, Default)]
pub struct SidebarItem {
    pub title: String,
    pub page_id: String,
    pub children: Vec<SidebarItem>,
}

#[derive(Debug, Clone, Default)]
pub struct Wiki {
    pub pages: Vec<WikiPage>,
    pub sidebar: Vec<SidebarItem>,
    pub project_name: String,
}

impl Wiki {
    pub fn get_page(&self, page_id: &str) -> Option<&WikiPage> {
        self.pages.iter().find(|p| p.id == page_id)
    }
}

pub struct WikiBuilder;

impl WikiBuilder {
    pub fn build(
        &self,
        project: &ProjectContext,
        wiki_data: &WikiData,
        graph: &DependencyGraph,
    ) -> Wiki {
        let mut pages: Vec<WikiPage> = Vec::new();
        let mut sidebar: Vec<SidebarItem> = Vec::new();

        let overview = &wiki_data.overview;
        let overview_md = build_overview_page(overview, project);
        pages.push(WikiPage {
            id: "index".into(),
            title: "Overview".into(),
            content: overview_md,
            order: 0,
            ..Default::default()
        });
        sidebar.push(SidebarItem {
            title: "Overview".into(),
            page_id: "index".into(),
            ..Default::default()
        });

        let arch = &wiki_data.architecture;
        if !arch.architecture_type.is_empty() {
            let module_names: std::collections::HashSet<String> =
                wiki_data.modules.iter().map(|m| m.name.clone()).collect();
            let arch_md = build_architecture_page(arch, &module_names);
            pages.push(WikiPage {
                id: "architecture".into(),
                title: "Architecture".into(),
                content: arch_md,
                order: 1,
                ..Default::default()
            });
            sidebar.push(SidebarItem {
                title: "Architecture".into(),
                page_id: "architecture".into(),
                ..Default::default()
            });
        }

        let mut module_sidebar = SidebarItem {
            title: "Modules".into(),
            ..Default::default()
        };
        for (i, mod_doc) in wiki_data.modules.iter().enumerate() {
            let mod_id = format!("modules/{}", mod_doc.name);
            let mod_md = build_module_page(mod_doc);
            pages.push(WikiPage {
                id: mod_id.clone(),
                title: mod_doc.name.clone(),
                content: mod_md,
                parent_id: "modules".into(),
                order: i as i32,
            });
            module_sidebar.children.push(SidebarItem {
                title: mod_doc.name.clone(),
                page_id: mod_id,
                ..Default::default()
            });
        }
        if !module_sidebar.children.is_empty() {
            sidebar.push(module_sidebar);
        }

        let guide = &wiki_data.reading_guide;
        if !guide.steps.is_empty() {
            let guide_md = build_reading_guide_page(guide);
            pages.push(WikiPage {
                id: "reading-guide".into(),
                title: "Reading Guide".into(),
                content: guide_md,
                order: 10,
                ..Default::default()
            });
            sidebar.push(SidebarItem {
                title: "Reading Guide".into(),
                page_id: "reading-guide".into(),
                ..Default::default()
            });
        }

        let mermaid = graph.to_mermaid();
        if !mermaid.is_empty() {
            let dep_md = build_dependency_page(graph, &mermaid);
            pages.push(WikiPage {
                id: "dependencies".into(),
                title: "Dependencies".into(),
                content: dep_md,
                order: 11,
                ..Default::default()
            });
            sidebar.push(SidebarItem {
                title: "Dependencies".into(),
                page_id: "dependencies".into(),
                ..Default::default()
            });
        }

        let symbols_md = build_symbol_index_page(wiki_data);
        if !symbols_md.is_empty() {
            pages.push(WikiPage {
                id: "symbols".into(),
                title: "Symbol Index".into(),
                content: symbols_md,
                order: 12,
                ..Default::default()
            });
            sidebar.push(SidebarItem {
                title: "Symbol Index".into(),
                page_id: "symbols".into(),
                ..Default::default()
            });
        }

        link_pages(&mut pages, wiki_data);
        Wiki {
            pages,
            sidebar,
            project_name: project.name.clone(),
        }
    }
}

fn link_pages(pages: &mut [WikiPage], wiki_data: &WikiData) {
    let mut symbols_raw: HashMap<String, Option<String>> = HashMap::new();
    let mut files_raw: HashMap<String, Option<String>> = HashMap::new();
    for mod_doc in &wiki_data.modules {
        let page_id = format!("modules/{}", mod_doc.name);
        for f in &mod_doc.files {
            match files_raw.get_mut(&f.path) {
                Some(slot) => *slot = None,
                None => { files_raw.insert(f.path.clone(), Some(page_id.clone())); }
            }
            for s in &f.key_symbols {
                match symbols_raw.get_mut(&s.name) {
                    Some(slot) => *slot = None,
                    None => { symbols_raw.insert(s.name.clone(), Some(page_id.clone())); }
                }
            }
        }
    }
    let files: HashMap<String, String> = files_raw.into_iter()
        .filter_map(|(k, v)| v.map(|id| (k, id)))
        .collect();
    let symbols: HashMap<String, String> = symbols_raw.into_iter()
        .filter_map(|(k, v)| v.map(|id| (k, id)))
        .collect();
    if symbols.is_empty() && files.is_empty() {
        return;
    }
    for page in pages.iter_mut() {
        page.content = apply_cross_links(&page.content, &page.id, &symbols, &files);
        if page.id.starts_with("modules/") {
            page.content = strip_relationship_links(&page.content);
        }
    }
}

fn build_overview_page(overview: &ProjectOverview, project: &ProjectContext) -> String {
    let mut lines = Vec::new();
    let name = if overview.name.is_empty() {
        &project.name
    } else {
        &overview.name
    };
    lines.push(format!("# {}\n", name));
    if !overview.one_liner.is_empty() {
        lines.push(format!("> {}\n", overview.one_liner));
    }
    if !overview.description.is_empty() {
        lines.push(format!("{}\n", overview.description));
    }

    if let Some(ref coverage) = project.coverage {
        if coverage.partial() {
            let mut note = format!(
                "> **Partial coverage:** this wiki was built from {}.",
                coverage.summary_line()
            );
            if !coverage.oversized.is_empty() {
                let paths: Vec<String> = coverage.oversized.iter().map(|p| format!("`{}`", p)).collect();
                note.push_str(&format!(" Oversized files left out: {}.", paths.join(", ")));
            }
            if !coverage.skipped_dirs.is_empty() {
                let dirs: Vec<String> = coverage.skipped_dirs.iter().take(5).map(|d| format!("`{}`", d)).collect();
                note.push_str(&format!(" Excluded directories: {}.", dirs.join(", ")));
            }
            lines.push(format!("{} Pages below describe only the scanned subset.\n", note));
        }
    }

    if !overview.tech_stack.is_empty() {
        lines.push("## Tech Stack\n".into());
        for t in &overview.tech_stack {
            let ver = if t.version.is_empty() { String::new() } else { format!(" {}", t.version) };
            let cat = if t.category.is_empty() { String::new() } else { format!(" ({})", t.category) };
            lines.push(format!("- **{}**{}{}", t.name, ver, cat));
        }
        lines.push(String::new());
    }

    if !overview.key_features.is_empty() {
        lines.push("## Key Features\n".into());
        for feat in &overview.key_features {
            lines.push(format!("- {}", feat));
        }
        lines.push(String::new());
    }

    if !overview.setup_instructions.is_empty() {
        lines.push("## Getting Started\n".into());
        for (i, step) in overview.setup_instructions.iter().enumerate() {
            lines.push(format!("{}. {}", i + 1, step));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn build_architecture_page(arch: &ArchitectureDiagram, module_names: &std::collections::HashSet<String>) -> String {
    let mut lines = Vec::new();
    lines.push("# Architecture\n".into());
    if !arch.architecture_type.is_empty() {
        lines.push(format!("**Type:** {}\n", arch.architecture_type));
    }
    if !arch.description.is_empty() {
        lines.push(format!("{}\n", arch.description));
    }
    if !arch.mermaid_component.is_empty() {
        lines.push("## Component Diagram\n".into());
        lines.push(format!("```mermaid\n{}\n```\n", arch.mermaid_component));
    }
    if !arch.components.is_empty() {
        lines.push("## Components\n".into());
        for c in &arch.components {
            lines.push(format!("### {}\n", c.name));
            if !c.purpose.is_empty() {
                lines.push(format!("{}\n", c.purpose));
            }
            if !c.files.is_empty() {
                let page_id = format!("modules/{}", c.name);
                let has_module = module_names.contains(&c.name);
                let files: Vec<String> = c.files.iter().map(|f| {
                    if has_module {
                        format!("[`{}`]({})", f, rel_href("architecture", &page_id))
                    } else {
                        format!("`{}`", f)
                    }
                }).collect();
                lines.push(format!("Files: {}\n", files.join(", ")));
            }
        }
    }
    if !arch.mermaid_sequence.is_empty() {
        lines.push("## Sequence Diagram\n".into());
        lines.push(format!("```mermaid\n{}\n```\n", arch.mermaid_sequence));
    }
    if !arch.data_flow.is_empty() {
        lines.push("## Data Flow\n".into());
        lines.push(format!("{}\n", arch.data_flow));
    }
    lines.join("\n")
}

fn build_module_page(mod_doc: &ModuleDoc) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# {}\n", mod_doc.name));
    if !mod_doc.purpose.is_empty() {
        lines.push(format!("> {}\n", mod_doc.purpose));
    }
    if !mod_doc.description.is_empty() {
        lines.push(format!("{}\n", mod_doc.description));
    }
    if !mod_doc.files.is_empty() {
        lines.push("## Files\n".into());
        for f in &mod_doc.files {
            lines.push(format!("### `{}`\n", f.path));
            if !f.purpose.is_empty() {
                lines.push(format!("{}\n", f.purpose));
            }
            if !f.key_symbols.is_empty() {
                for s in &f.key_symbols {
                    let desc = if s.description.is_empty() {
                        String::new()
                    } else {
                        format!(" - {}", s.description)
                    };
                    lines.push(format!("- `{}` ({}){}", s.name, s.kind, desc));
                }
                lines.push(String::new());
            }
        }
    }
    if !mod_doc.key_concepts.is_empty() {
        lines.push("## Key Concepts\n".into());
        for c in &mod_doc.key_concepts {
            lines.push(format!("- **{}**: {}", c.name, c.explanation));
        }
        lines.push(String::new());
    }
    if !mod_doc.relationships.is_empty() {
        lines.push("## Internal Relationships\n".into());
        for r in &mod_doc.relationships {
            lines.push(format!("- `{}` \u{2192} `{}`: {}", r.source, r.target, r.description));
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

fn build_symbol_index_page(wiki_data: &WikiData) -> String {
    let mut kinds: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let mut total = 0u32;

    for mod_doc in &wiki_data.modules {
        for f in &mod_doc.files {
            for s in &f.key_symbols {
                let kind = if s.kind.is_empty() { "other".to_string() } else { s.kind.clone() };
                let bucket = kinds
                    .entry(kind)
                    .or_default()
                    .entry(mod_doc.name.clone())
                    .or_default();
                if !bucket.contains_key(&s.name) {
                    bucket.insert(s.name.clone(), s.description.clone());
                    total += 1;
                }
            }
        }
    }

    if kinds.is_empty() {
        return String::new();
    }

    let module_count: usize = kinds
        .values()
        .flat_map(|sections| sections.keys())
        .collect::<std::collections::HashSet<_>>()
        .len();

    let mut lines = Vec::new();
    lines.push("# Symbol Index\n".into());
    lines.push(format!("{} symbols across {} modules.\n", total, module_count));

    let mut kind_keys: Vec<&String> = kinds.keys().collect();
    kind_keys.sort();
    for kind in kind_keys {
        let modules = &kinds[kind];
        let mut header = kind.clone();
        if let Some(first) = header.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        lines.push(format!("## {}\n", header));

        let mut mod_names: Vec<&String> = modules.keys().collect();
        mod_names.sort();
        for mod_name in mod_names {
            let href = rel_href("symbols", &format!("modules/{}", mod_name));
            lines.push(format!("### [{}]({})\n", mod_name, href));

            let bucket = &modules[mod_name];
            let mut sym_names: Vec<&String> = bucket.keys().collect();
            sym_names.sort();
            for sym_name in sym_names {
                let desc = &bucket[sym_name];
                let suffix = if desc.is_empty() {
                    String::new()
                } else {
                    format!(" - {}", desc)
                };
                lines.push(format!("- [`{}`]({}){}", sym_name, href, suffix));
            }
            lines.push(String::new());
        }
    }
    lines.join("\n")
}

fn build_reading_guide_page(guide: &ReadingGuide) -> String {
    let mut lines = Vec::new();
    lines.push("# Reading Guide\n".into());
    if !guide.introduction.is_empty() {
        lines.push(format!("{}\n", guide.introduction));
    }
    for step in &guide.steps {
        let time_est = if step.time_estimate.is_empty() {
            String::new()
        } else {
            format!(" (~{})", step.time_estimate)
        };
        lines.push(format!("## Step {}: {}{}\n", step.order, step.title, time_est));
        if !step.files.is_empty() {
            let files: Vec<String> = step.files.iter().map(|f| format!("`{}`", f)).collect();
            lines.push(format!("**Files:** {}\n", files.join(", ")));
        }
        if !step.explanation.is_empty() {
            lines.push(format!("{}\n", step.explanation));
        }
    }
    if !guide.tips.is_empty() {
        lines.push("## Tips\n".into());
        for tip in &guide.tips {
            lines.push(format!("- {}", tip));
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

fn build_dependency_page(graph: &DependencyGraph, mermaid: &str) -> String {
    let mut lines = Vec::new();
    lines.push("# Module Dependencies\n".into());
    lines.push(format!("```mermaid\n{}\n```\n", mermaid));

    let core = graph.get_core_files(10);
    if !core.is_empty() {
        lines.push("## Core Files (by PageRank)\n".into());
        for (i, path) in core.iter().enumerate() {
            lines.push(format!("{}. `{}`", i + 1, path));
        }
        lines.push(String::new());
    }

    let entries = graph.get_entry_points();
    if !entries.is_empty() {
        lines.push("## Likely Entry Points\n".into());
        for e in entries.iter().take(10) {
            lines.push(format!("- `{}`", e));
        }
        lines.push(String::new());
    }

    let cycles = graph.find_circular_dependencies(10);
    if !cycles.is_empty() {
        lines.push("## Circular Dependencies\n".into());
        lines.push(
            "These groups of files import each other in a cycle, so you can't \
             fully understand one without the others; consider breaking the loop \
             to reduce coupling.\n"
                .into(),
        );
        for cycle in &cycles {
            let files: Vec<String> = cycle.iter().map(|p| format!("`{}`", p)).collect();
            lines.push(format!("- {}", files.join(", ")));
        }
        lines.push(String::new());
    }

    let isolated = graph.find_isolated_files();
    if !isolated.is_empty() {
        lines.push("## Isolated Files\n".into());
        lines.push(
            "These files import nothing in the project and are imported by \
             nothing -- likely dead code, stray scripts, or modules that were \
             never wired in.\n"
                .into(),
        );
        for f in isolated.iter().take(15) {
            lines.push(format!("- `{}`", f));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn apply_cross_links(
    content: &str,
    page_id: &str,
    symbols: &HashMap<String, String>,
    files: &HashMap<String, String>,
) -> String {
    let code_span = Regex::new(r"`([^`\n]+)`").unwrap();
    let mut out = Vec::new();
    let mut in_fence = false;

    for line in content.split('\n') {
        if line.starts_with("```") {
            in_fence = !in_fence;
            out.push(line.to_string());
            continue;
        }
        if in_fence {
            out.push(line.to_string());
            continue;
        }
        out.push(linkify_line(line, page_id, symbols, files, &code_span));
    }
    out.join("\n")
}

fn strip_relationship_links(content: &str) -> String {
    let section_header = "## Internal Relationships";
    let Some(section_start) = content.find(section_header) else {
        return content.to_string();
    };
    let link_re = Regex::new(r"\[(`[^`\n]+`)\]\([^)]+\)").unwrap();
    let before = &content[..section_start];
    let section = &content[section_start..];
    let stripped = link_re.replace_all(section, "$1").to_string();
    format!("{}{}", before, stripped)
}

fn linkify_line(
    line: &str,
    page_id: &str,
    symbols: &HashMap<String, String>,
    files: &HashMap<String, String>,
    code_span: &Regex,
) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    for mat in code_span.find_iter(line) {
        let start = mat.start();
        let end = mat.end();
        let matched = mat.as_str();
        let name = &matched[1..matched.len() - 1];

        let before = if start > 0 { line.as_bytes()[start - 1] } else { 0 };
        let after = if end < line.len() { line.as_bytes()[end] } else { 0 };
        let already_linked = before == b'[' || (after == b']' && end + 1 < line.len() && line.as_bytes()[end + 1] == b'(');

        if already_linked {
            result.push_str(&line[last_end..end]);
        } else {
            let target = files.get(name).or_else(|| symbols.get(name));
            match target {
                Some(t) if t != page_id => {
                    result.push_str(&line[last_end..start]);
                    result.push_str(&format!("[`{}`]({})", name, rel_href(page_id, t)));
                }
                _ => {
                    result.push_str(&line[last_end..end]);
                }
            }
        }
        last_end = end;
    }
    result.push_str(&line[last_end..]);
    result
}

fn rel_href(from_page_id: &str, to_page_id: &str) -> String {
    let target = format!("{}.md", to_page_id);
    let base = from_page_id.rfind('/').map(|i| &from_page_id[..i]);
    match base {
        Some("") | None => target,
        Some(base) => {
            let base_parts: Vec<&str> = base.split('/').collect();
            let target_parts: Vec<&str> = to_page_id.split('/').collect();
            let common = base_parts
                .iter()
                .zip(target_parts.iter())
                .take_while(|(a, b)| a == b)
                .count();
            let ups = base_parts.len() - common;
            let mut path = String::new();
            for _ in 0..ups {
                path.push_str("../");
            }
            let remaining: Vec<&str> = target_parts[common..].to_vec();
            path.push_str(&remaining.join("/"));
            format!("{}.md", path)
        }
    }
}
