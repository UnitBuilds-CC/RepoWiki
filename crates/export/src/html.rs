use std::collections::HashSet;
use std::path::Path;

use regex::Regex;
use repowiki_wiki::Wiki;

pub fn export_html(wiki: &Wiki, output_path: &Path) -> bool {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let page_ids: HashSet<String> = wiki.pages.iter().map(|p| p.id.clone()).collect();
    let mut pages_html = Vec::new();
    let mut nav_html = Vec::new();

    for item in &wiki.sidebar {
        if !item.page_id.is_empty() {
            nav_html.push(format!(
                "<a class=\"nav-item\" href=\"#\" onclick=\"showPage('{}')\">{}</a>",
                item.page_id,
                html_escape(&item.title)
            ));
        } else {
            nav_html.push(format!(
                "<div class=\"nav-group\">{}</div>",
                html_escape(&item.title)
            ));
        }
        for child in &item.children {
            nav_html.push(format!(
                "<a class=\"nav-item nav-child\" href=\"#\" onclick=\"showPage('{}')\">{}</a>",
                child.page_id,
                html_escape(&child.title)
            ));
        }
    }

    for page in &wiki.pages {
        let content = markdown_to_html(&page.content, &page.id, &page_ids);
        pages_html.push(format!(
            "<div id=\"page-{}\" class=\"wiki-page\" style=\"display:none\">{}</div>",
            page.id, content
        ));
    }

    let first_page = wiki.pages.first().map(|p| p.id.as_str()).unwrap_or("index");
    let template = HTML_TEMPLATE
        .replace("{title}", &html_escape(&wiki.project_name))
        .replace("{nav}", &nav_html.join(""))
        .replace("{pages}", &pages_html.join(""))
        .replace("{first_page}", first_page);

    if let Ok(existing) = std::fs::read_to_string(output_path) {
        if existing == template {
            return false;
        }
    }
    std::fs::write(output_path, &template).ok();
    true
}

fn markdown_to_html(md: &str, page_id: &str, page_ids: &HashSet<String>) -> String {
    let mut result = Vec::new();
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_lines: Vec<String> = Vec::new();
    let mut list_type: Option<&str> = None;

    for line in md.split('\n') {
        if line.starts_with("```") {
            if in_code {
                let code = html_escape(&code_lines.join("\n"));
                if code_lang == "mermaid" {
                    result.push(format!("<div class=\"mermaid\">{}</div>", code));
                } else {
                    result.push(format!(
                        "<pre><code class=\"language-{}\">{}</code></pre>",
                        code_lang, code
                    ));
                }
                code_lines.clear();
                in_code = false;
            } else {
                if list_type.is_some() {
                    result.push(format!("</{}>", list_type.unwrap()));
                    list_type = None;
                }
                in_code = true;
                code_lang = line[3..].trim().to_string();
                if code_lang.is_empty() {
                    code_lang = "text".to_string();
                }
            }
            continue;
        }

        if in_code {
            code_lines.push(line.to_string());
            continue;
        }

        if line.starts_with("- ") {
            if list_type != Some("ul") {
                if let Some(lt) = list_type {
                    result.push(format!("</{}>", lt));
                }
                result.push("<ul>".to_string());
                list_type = Some("ul");
            }
            result.push(format!(
                "<li>{}</li>",
                inline_md(&line[2..], page_id, page_ids)
            ));
            continue;
        }

        let ol_re = Regex::new(r"^\d+\. ").unwrap();
        if ol_re.is_match(line) {
            if list_type != Some("ol") {
                if let Some(lt) = list_type {
                    result.push(format!("</{}>", lt));
                }
                result.push("<ol>".to_string());
                list_type = Some("ol");
            }
            let text = ol_re.replace(line, "");
            result.push(format!("<li>{}</li>", inline_md(&text, page_id, page_ids)));
            continue;
        }

        if list_type.is_some() {
            result.push(format!("</{}>", list_type.unwrap()));
            list_type = None;
        }

        if line.starts_with("# ") {
            result.push(format!(
                "<h1>{}</h1>",
                inline_md(&line[2..], page_id, page_ids)
            ));
        } else if line.starts_with("## ") {
            result.push(format!(
                "<h2>{}</h2>",
                inline_md(&line[3..], page_id, page_ids)
            ));
        } else if line.starts_with("### ") {
            result.push(format!(
                "<h3>{}</h3>",
                inline_md(&line[4..], page_id, page_ids)
            ));
        } else if line.starts_with("> ") {
            result.push(format!(
                "<blockquote>{}</blockquote>",
                inline_md(&line[2..], page_id, page_ids)
            ));
        } else if line.trim().is_empty() {
            result.push("<br>".to_string());
        } else {
            result.push(format!(
                "<p>{}</p>",
                inline_md(line, page_id, page_ids)
            ));
        }
    }

    if let Some(lt) = list_type {
        result.push(format!("</{}>", lt));
    }

    result.join("\n")
}

fn inline_md(text: &str, page_id: &str, page_ids: &HashSet<String>) -> String {
    let mut text = html_escape(text);

    let bold_re = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    text = bold_re.replace_all(&text, "<strong>$1</strong>").to_string();

    let code_re = Regex::new(r"`(.+?)`").unwrap();
    text = code_re.replace_all(&text, "<code>$1</code>").to_string();

    let link_re = Regex::new(r"\[(.+?)\]\((.+?)\)").unwrap();
    text = link_re
        .replace_all(&text, |caps: &regex::Captures| {
            let label = &caps[1];
            let target = &caps[2];
            if target.ends_with(".md") {
                let base = page_id.rfind('/').map(|i| &page_id[..i]).unwrap_or("");
                let joined = if base.is_empty() {
                    target.to_string()
                } else {
                    format!("{}/{}", base, target)
                };
                let resolved = normalize_path(&joined);
                let resolved = resolved.strip_suffix(".md").unwrap_or(&resolved);
                if page_ids.contains(resolved) {
                    return format!(
                        "<a href=\"#\" onclick=\"showPage('{}')\">{}</a>",
                        resolved, label
                    );
                }
            }
            format!("<a href=\"{}\">{}</a>", target, label)
        })
        .to_string();

    text
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

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} - RepoWiki</title>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  display: flex; height: 100vh; color: #1a1a1a; background: #fff; }
.sidebar { width: 260px; border-right: 1px solid #e5e7eb; padding: 16px;
  overflow-y: auto; flex-shrink: 0; background: #fafafa; }
.sidebar h2 { font-size: 16px; margin-bottom: 12px; color: #111; }
.nav-item { display: block; padding: 6px 12px; color: #374151; text-decoration: none;
  border-radius: 6px; font-size: 14px; cursor: pointer; }
.nav-item:hover { background: #e5e7eb; }
.nav-item.active { background: #dbeafe; color: #1d4ed8; font-weight: 500; }
.nav-child { padding-left: 28px; font-size: 13px; }
.nav-group { padding: 8px 12px 4px; font-size: 12px; font-weight: 600;
  text-transform: uppercase; color: #6b7280; margin-top: 8px; }
.content { flex: 1; padding: 32px 48px; overflow-y: auto; max-width: 900px; }
h1 { font-size: 28px; margin-bottom: 16px; border-bottom: 1px solid #e5e7eb; padding-bottom: 8px; }
h2 { font-size: 22px; margin: 24px 0 12px; }
h3 { font-size: 18px; margin: 20px 0 8px; }
p { margin: 8px 0; line-height: 1.7; }
li { margin: 4px 0 4px 24px; line-height: 1.6; }
blockquote { border-left: 3px solid #3b82f6; padding: 8px 16px; margin: 12px 0;
  background: #eff6ff; color: #1e40af; border-radius: 0 4px 4px 0; }
pre { background: #1e293b; color: #e2e8f0; padding: 16px; border-radius: 8px;
  overflow-x: auto; margin: 12px 0; font-size: 13px; line-height: 1.5; }
code { background: #f1f5f9; padding: 2px 6px; border-radius: 4px; font-size: 0.9em; }
pre code { background: none; padding: 0; }
strong { font-weight: 600; }
a { color: #2563eb; }
.mermaid { margin: 16px 0; text-align: center; }
</style>
</head>
<body>
<div class="sidebar">
  <h2>{title}</h2>
  {nav}
  <div style="margin-top: 24px; padding-top: 12px; border-top: 1px solid #e5e7eb;
    font-size: 11px; color: #9ca3af;">
    Generated by <a href="https://github.com/he-yufeng/RepoWiki" style="color:#6b7280">RepoWiki</a>
  </div>
</div>
<div class="content" id="content">
  {pages}
</div>
<script src="https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js"></script>
<script>
mermaid.initialize({ startOnLoad: false, theme: 'default' });
function showPage(id) {
  document.querySelectorAll('.wiki-page').forEach(p => p.style.display = 'none');
  document.querySelectorAll('.nav-item').forEach(a => a.classList.remove('active'));
  const page = document.getElementById('page-' + id);
  if (page) {
    page.style.display = 'block';
    mermaid.run({ nodes: page.querySelectorAll('.mermaid') });
  }
  event.target.classList.add('active');
}
showPage('{first_page}');
</script>
</body>
</html>"#;
