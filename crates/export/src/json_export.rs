use std::path::Path;

use serde::Serialize;

use repowiki_wiki::{SidebarItem, Wiki};

#[derive(Serialize)]
struct JsonExport<'a> {
    project_name: &'a str,
    pages: Vec<PageEntry<'a>>,
    sidebar: Vec<SidebarEntry<'a>>,
}

#[derive(Serialize)]
struct PageEntry<'a> {
    id: &'a str,
    title: &'a str,
    content: &'a str,
    parent_id: &'a str,
    order: i32,
}

#[derive(Serialize)]
struct SidebarEntry<'a> {
    title: &'a str,
    page_id: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<SidebarEntry<'a>>,
}

pub fn export_json(wiki: &Wiki, output_path: &Path) -> bool {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let data = JsonExport {
        project_name: &wiki.project_name,
        pages: wiki
            .pages
            .iter()
            .map(|p| PageEntry {
                id: &p.id,
                title: &p.title,
                content: &p.content,
                parent_id: &p.parent_id,
                order: p.order,
            })
            .collect(),
        sidebar: serialize_sidebar(&wiki.sidebar),
    };

    let text = serde_json::to_string_pretty(&data).unwrap_or_default() + "\n";
    if let Ok(existing) = std::fs::read_to_string(output_path) {
        if existing == text {
            return false;
        }
    }
    std::fs::write(output_path, &text).ok();
    true
}

fn serialize_sidebar(items: &[SidebarItem]) -> Vec<SidebarEntry<'_>> {
    items
        .iter()
        .map(|item| SidebarEntry {
            title: &item.title,
            page_id: &item.page_id,
            children: serialize_sidebar(&item.children),
        })
        .collect()
}
