use regex::Regex;
use repowiki_core::models::*;

pub fn extract_symbols(
    _path: &str,
    content: &str,
    language: &str,
) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    match language {
        "python" | "pyi" => extract_python(content),
        "rust" => extract_rust(content),
        "typescript" | "tsx" | "javascript" | "jsx" | "mjs" | "cjs" | "mts" => {
            extract_typescript(content)
        }
        "go" => extract_go(content),
        "java" => extract_java(content),
        _ => (Vec::new(), Vec::new()),
    }
}

pub fn compute_metrics(content: &str, language: &str) -> FileMetrics {
    let mut code_lines = 0;
    let mut comment_lines = 0;
    let mut blank_lines = 0;
    let mut complexity = 1;
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blank_lines += 1;
            continue;
        }

        let is_comment = match language {
            "python" | "pyi" => trimmed.starts_with('#'),
            "rust" | "go" | "java" | "typescript" | "tsx" | "javascript" | "jsx" | "mjs"
            | "cjs" | "mts" => {
                if in_block_comment {
                    if trimmed.contains("*/") {
                        in_block_comment = false;
                    }
                    true
                } else if trimmed.starts_with("//") {
                    true
                } else if trimmed.starts_with("/*") {
                    if !trimmed.contains("*/") {
                        in_block_comment = true;
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        if is_comment {
            comment_lines += 1;
        } else {
            code_lines += 1;
        }

        complexity += count_complexity(trimmed, language);
    }

    FileMetrics {
        code_lines,
        comment_lines,
        blank_lines,
        complexity,
    }
}

fn count_complexity(line: &str, language: &str) -> usize {
    let mut count = 0;
    let branch_keywords = match language {
        "python" | "pyi" => &["if ", "elif ", "for ", "while ", "except ", "and ", "or "][..],
        "rust" => &["if ", "else if ", "for ", "while ", "match ", "&&", "||"][..],
        "go" => &["if ", "else if ", "for ", "switch ", "case ", "&&", "||"][..],
        "java" => &["if ", "else if ", "for ", "while ", "switch ", "case ", "&&", "||"][..],
        "typescript" | "tsx" | "javascript" | "jsx" | "mjs" | "cjs" | "mts" => {
            &["if ", "else if ", "for ", "while ", "switch ", "case ", "&&", "||", "? "][..]
        }
        _ => &[][..],
    };

    for kw in branch_keywords {
        let mut start = 0;
        while let Some(pos) = line[start..].find(kw) {
            count += 1;
            start += pos + kw.len();
        }
    }
    count
}

// --- Python ---

fn extract_python(content: &str) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    let mut symbols = Vec::new();
    let imports = extract_python_imports(content);
    let lines: Vec<&str> = content.lines().collect();

    let re_class = Regex::new(r"^(\s*)class\s+(\w+)").unwrap();
    let re_func = Regex::new(r"^(\s*)(?:async\s+)?def\s+(\w+)\s*\(([^)]*)\)").unwrap();
    let re_decorator = Regex::new(r"^(\s*)@(\w+)").unwrap();
    let re_return = Regex::new(r"\)\s*->\s*(.+?)\s*:").unwrap();

    let mut pending_decorators: Vec<String> = Vec::new();
    let mut pending_decorator_indent: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        if let Some(cap) = re_decorator.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            if indent != pending_decorator_indent && !pending_decorators.is_empty() {
                pending_decorators.clear();
            }
            pending_decorator_indent = indent;
            pending_decorators.push(cap[2].to_string());
            continue;
        }

        if let Some(cap) = re_class.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            if indent == 0 || indent == pending_decorator_indent {
                let name = cap[2].to_string();
                let doc = extract_docstring(&lines, i);
                let end = find_block_end(&lines, i, indent);
                symbols.push(IndexedSymbol {
                    name,
                    kind: SymbolKind::Class,
                    line: i + 1,
                    end_line: end,
                    visibility: Visibility::Public,
                    doc_comment: doc,
                    decorators: std::mem::take(&mut pending_decorators),
                    fields: extract_class_fields(&lines, i, indent),
                    ..Default::default()
                });
                continue;
            }
        }

        if let Some(cap) = re_func.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let name = cap[2].to_string();
            let params_str = cap[3].to_string();
            let params = parse_python_params(&params_str);
            let return_type = re_return.captures(line).map(|c| c[1].trim().to_string());
            let doc = extract_docstring(&lines, i);
            let end = find_block_end(&lines, i, indent);

            let is_method = indent > 0;
            let kind = if is_method {
                SymbolKind::Method
            } else {
                SymbolKind::Function
            };

            let vis = if name.starts_with('_') && !name.starts_with("__") {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let calls = extract_python_calls(line);

            symbols.push(IndexedSymbol {
                name,
                kind,
                line: i + 1,
                end_line: end,
                visibility: vis,
                params,
                return_type,
                doc_comment: doc,
                decorators: std::mem::take(&mut pending_decorators),
                calls,
                ..Default::default()
            });
            continue;
        }

        if !trimmed.starts_with('@') {
            pending_decorators.clear();
        }
    }

    (symbols, imports)
}

fn extract_python_imports(content: &str) -> Vec<IndexedImport> {
    let mut imports = Vec::new();
    let re_import = Regex::new(r"(?m)^\s*import\s+([\w.]+)").unwrap();
    let re_from = Regex::new(r"(?m)^\s*from\s+([\w.]+)\s+import\s+(.+)").unwrap();

    for cap in re_import.captures_iter(content) {
        imports.push(IndexedImport {
            raw: format!("import {}", &cap[1]),
            symbols: Vec::new(),
            ..Default::default()
        });
    }

    for cap in re_from.captures_iter(content) {
        let module = &cap[1];
        let names_str = &cap[2];
        let symbols: Vec<String> = names_str
            .split(',')
            .map(|s| s.trim().split(" as ").next().unwrap_or("").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        imports.push(IndexedImport {
            raw: format!("from {} import ...", module),
            symbols,
            ..Default::default()
        });
    }

    imports
}

fn parse_python_params(params_str: &str) -> Vec<Parameter> {
    params_str
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p == "self" || p == "cls" {
                return None;
            }
            let (name, type_hint) = if let Some((n, t)) = p.split_once(':') {
                (n.trim().to_string(), Some(t.trim().to_string()))
            } else {
                let name = p.split('=').next().unwrap_or(p).trim().to_string();
                (name, None)
            };
            if name.is_empty() || name.starts_with('*') {
                return None;
            }
            Some(Parameter { name, type_hint })
        })
        .collect()
}

fn extract_docstring(lines: &[&str], def_line: usize) -> Option<String> {
    if def_line + 1 >= lines.len() {
        return None;
    }
    let next = lines[def_line + 1].trim();
    if next.starts_with("\"\"\"") || next.starts_with("'''") {
        let quote = &next[..3];
        if next.len() > 3 && next.ends_with(quote) {
            return Some(next[3..next.len() - 3].trim().to_string());
        }
        let mut doc_lines = Vec::new();
        let first = &next[3..];
        if !first.is_empty() {
            doc_lines.push(first.to_string());
        }
        for j in (def_line + 2)..lines.len() {
            let l = lines[j].trim();
            if l.ends_with(quote) {
                let end = l.len() - 3;
                if end > 0 {
                    doc_lines.push(l[..end].to_string());
                }
                break;
            }
            doc_lines.push(l.to_string());
        }
        let doc = doc_lines.join("\n").trim().to_string();
        if doc.is_empty() {
            None
        } else {
            Some(doc)
        }
    } else {
        None
    }
}

fn extract_class_fields(lines: &[&str], class_line: usize, class_indent: usize) -> Vec<String> {
    let mut fields = Vec::new();
    let re_init = Regex::new(r"^\s+def\s+__init__").unwrap();
    let re_self_assign = Regex::new(r"self\.(\w+)\s*=").unwrap();

    let mut in_init = false;
    let mut init_indent = 0;

    for i in (class_line + 1)..lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        if indent <= class_indent && !trimmed.is_empty() {
            break;
        }

        if re_init.is_match(line) {
            in_init = true;
            init_indent = indent;
            continue;
        }

        if in_init && indent <= init_indent && !trimmed.is_empty() {
            break;
        }

        if in_init {
            for cap in re_self_assign.captures_iter(line) {
                fields.push(cap[1].to_string());
            }
        }
    }

    fields
}

fn extract_python_calls(line: &str) -> Vec<String> {
    let re_call = Regex::new(r"(\w+)\s*\(").unwrap();
    let keywords = [
        "if", "for", "while", "def", "class", "return", "import", "from", "with", "as", "try",
        "except", "raise", "yield", "print", "async", "await", "lambda", "not", "and", "or",
        "in", "is", "else", "elif", "pass", "break", "continue",
    ];
    re_call
        .captures_iter(line)
        .map(|c| c[1].to_string())
        .filter(|name| !keywords.contains(&name.as_str()))
        .collect()
}

fn find_block_end(lines: &[&str], start: usize, block_indent: usize) -> usize {
    let mut last_code = start;
    for i in (start + 1)..lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent <= block_indent {
            break;
        }
        last_code = i;
    }
    last_code + 1
}

// --- Rust ---

fn extract_rust(content: &str) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    let mut symbols = Vec::new();
    let imports = extract_rust_imports(content);
    let lines: Vec<&str> = content.lines().collect();

    let re_fn = Regex::new(r#"^(\s*)(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern(?:\s+"[^"]*")?\s+)?fn\s+(\w+)"#).unwrap();
    let re_struct = Regex::new(r"^(\s*)(pub(?:\([^)]*\))?\s+)?struct\s+(\w+)").unwrap();
    let re_enum = Regex::new(r"^(\s*)(pub(?:\([^)]*\))?\s+)?enum\s+(\w+)").unwrap();
    let re_trait = Regex::new(r"^(\s*)(pub(?:\([^)]*\))?\s+)?trait\s+(\w+)").unwrap();
    let re_const = Regex::new(r"^(\s*)(pub(?:\([^)]*\))?\s+)?(?:const|static)\s+(\w+)").unwrap();
    let re_macro = Regex::new(r"macro_rules!\s*(\w+)").unwrap();
    let re_type = Regex::new(r"^(\s*)(pub(?:\([^)]*\))?\s+)?type\s+(\w+)").unwrap();
    let re_impl = Regex::new(r"^(\s*)impl(?:<[^>]*>)?\s+(\w+)").unwrap();
    let re_returns = Regex::new(r"\)\s*->\s*(.+)?\s*\{").unwrap();

    let mut in_impl: Option<String> = None;
    let mut impl_indent: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }

        if let Some(cap) = re_impl.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            in_impl = Some(cap[2].to_string());
            impl_indent = indent;
            continue;
        }

        if let Some(cap) = re_fn.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();

            if let Some(ref _impl_type) = in_impl {
                if indent > impl_indent {
                    let doc = extract_rust_doc(&lines, i);
                    let end = find_rust_block_end(&lines, i, indent);
                    let params = extract_rust_fn_params(line);
                    let return_type = re_returns.captures(line).map(|c| c[1].trim().to_string());
                    symbols.push(IndexedSymbol {
                        name,
                        kind: SymbolKind::Method,
                        line: i + 1,
                        end_line: end,
                        visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                        params,
                        return_type,
                        doc_comment: doc,
                        ..Default::default()
                    });
                    continue;
                }
            }

            in_impl = None;
            let doc = extract_rust_doc(&lines, i);
            let end = find_rust_block_end(&lines, i, indent);
            let params = extract_rust_fn_params(line);
            let return_type = re_returns.captures(line).map(|c| c[1].trim().to_string());

            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Function,
                line: i + 1,
                end_line: end,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                params,
                return_type,
                doc_comment: doc,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_struct.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_impl = None;
            let doc = extract_rust_doc(&lines, i);
            let end = find_rust_block_end(&lines, i, indent);
            let fields = extract_rust_struct_fields(&lines, i, indent);
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Struct,
                line: i + 1,
                end_line: end,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                doc_comment: doc,
                fields,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_enum.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_impl = None;
            let doc = extract_rust_doc(&lines, i);
            let end = find_rust_block_end(&lines, i, indent);
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Enum,
                line: i + 1,
                end_line: end,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                doc_comment: doc,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_trait.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_impl = None;
            let doc = extract_rust_doc(&lines, i);
            let end = find_rust_block_end(&lines, i, indent);
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Trait,
                line: i + 1,
                end_line: end,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                doc_comment: doc,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_const.captures(line) {
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_impl = None;
            let doc = extract_rust_doc(&lines, i);
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Constant,
                line: i + 1,
                end_line: i + 1,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                doc_comment: doc,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_type.captures(line) {
            let is_pub = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_impl = None;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::TypeAlias,
                line: i + 1,
                end_line: i + 1,
                visibility: if is_pub { Visibility::Public } else { Visibility::Private },
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_macro.captures(line) {
            let name = cap[1].to_string();
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Macro,
                line: i + 1,
                end_line: i + 1,
                visibility: Visibility::Public,
                ..Default::default()
            });
        }
    }

    (symbols, imports)
}

fn extract_rust_imports(content: &str) -> Vec<IndexedImport> {
    let mut imports = Vec::new();
    let re_use = Regex::new(r"(?m)^\s*use\s+([\w:]+(?:\s*\{[^}]*\})?)").unwrap();

    for cap in re_use.captures_iter(content) {
        let full = cap[1].trim().to_string();
        let symbols = if full.contains('{') {
            let inner = full.split('{').nth(1).unwrap_or("");
            let inner = inner.trim_end_matches('}');
            inner
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        };
        imports.push(IndexedImport {
            raw: full,
            symbols,
            ..Default::default()
        });
    }

    imports
}

fn extract_rust_doc(lines: &[&str], def_line: usize) -> Option<String> {
    let mut doc_lines = Vec::new();
    let mut i = def_line;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            let content = trimmed.trim_start_matches('/').trim_start_matches('!').trim();
            doc_lines.push(content.to_string());
        } else if trimmed.is_empty() {
            continue;
        } else {
            break;
        }
    }
    doc_lines.reverse();
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}

fn extract_rust_fn_params(line: &str) -> Vec<Parameter> {
    let re_params = Regex::new(r"\(([^)]*)\)").unwrap();
    let cap = match re_params.captures(line) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let params_str = &cap[1];
    params_str
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p == "self" || p == "&self" || p == "&mut self" {
                return None;
            }
            let p = p.trim_start_matches("mut ");
            if let Some((name, type_hint)) = p.split_once(':') {
                let name = name.trim().to_string();
                let type_hint = type_hint.trim().to_string();
                if name.is_empty() {
                    return None;
                }
                Some(Parameter {
                    name,
                    type_hint: Some(type_hint),
                })
            } else {
                None
            }
        })
        .collect()
}

fn extract_rust_struct_fields(lines: &[&str], struct_line: usize, struct_indent: usize) -> Vec<String> {
    let mut fields = Vec::new();
    let re_field = Regex::new(r"^\s*(pub(?:\([^)]*\))?\s+)?(\w+)\s*:").unwrap();

    for i in (struct_line + 1)..lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent <= struct_indent {
            break;
        }
        if trimmed == "}" || trimmed.starts_with('}') {
            break;
        }
        if let Some(cap) = re_field.captures(line) {
            fields.push(cap[2].to_string());
        }
    }

    fields
}

fn find_rust_block_end(lines: &[&str], start: usize, _block_indent: usize) -> usize {
    let mut depth = 0i32;
    let mut found_open = false;
    for i in start..lines.len() {
        for ch in lines[i].chars() {
            if ch == '{' {
                depth += 1;
                found_open = true;
            } else if ch == '}' {
                depth -= 1;
                if found_open && depth == 0 {
                    return i + 1;
                }
            }
        }
    }
    start + 1
}

// --- TypeScript / JavaScript ---

fn extract_typescript(content: &str) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    let mut symbols = Vec::new();
    let imports = extract_ts_imports(content);
    let lines: Vec<&str> = content.lines().collect();

    let re_func = Regex::new(r"^(\s*)(export\s+)?(?:async\s+)?function\s+(\w+)").unwrap();
    let re_class = Regex::new(r"^(\s*)(export\s+)?(?:abstract\s+)?class\s+(\w+)").unwrap();
    let re_interface = Regex::new(r"^(\s*)(export\s+)?interface\s+(\w+)").unwrap();
    let re_type = Regex::new(r"^(\s*)(export\s+)?type\s+(\w+)").unwrap();
    let re_const_fn =
        Regex::new(r"^(\s*)(export\s+)?(?:const|let)\s+(\w+)\s*(?::\s*\w+)?\s*=\s*(?:async\s+)?\(")
            .unwrap();
    let re_const_arrow =
        Regex::new(r"^(\s*)(export\s+)?(?:const|let)\s+(\w+)\s*(?::\s*\w+)?\s*=\s*(?:async\s+)?\w+\s*=>")
            .unwrap();
    let re_method = Regex::new(r"^\s+(public|private|protected)?\s*(?:async\s+)?(\w+)\s*\(").unwrap();

    let mut in_class = false;
    let mut class_indent: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }

        if let Some(cap) = re_class.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_export = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_class = true;
            class_indent = indent;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Class,
                line: i + 1,
                end_line: find_ts_block_end(&lines, i),
                visibility: if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_interface.captures(line) {
            let is_export = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_class = false;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Interface,
                line: i + 1,
                end_line: find_ts_block_end(&lines, i),
                visibility: if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_type.captures(line) {
            let is_export = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_class = false;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::TypeAlias,
                line: i + 1,
                end_line: i + 1,
                visibility: if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_func.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let is_export = cap.get(2).is_some();
            let name = cap[3].to_string();

            if in_class && indent > class_indent {
                let kind = SymbolKind::Method;
                let params = extract_ts_params(line);
                symbols.push(IndexedSymbol {
                    name,
                    kind,
                    line: i + 1,
                    end_line: find_ts_block_end(&lines, i),
                    visibility: if is_export {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    params,
                    ..Default::default()
                });
            } else {
                in_class = false;
                let params = extract_ts_params(line);
                symbols.push(IndexedSymbol {
                    name,
                    kind: SymbolKind::Function,
                    line: i + 1,
                    end_line: find_ts_block_end(&lines, i),
                    visibility: if is_export {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    params,
                    ..Default::default()
                });
            }
            continue;
        }

        if let Some(cap) = re_const_fn.captures(line).or_else(|| re_const_arrow.captures(line)) {
            let is_export = cap.get(2).is_some();
            let name = cap[3].to_string();
            in_class = false;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Function,
                line: i + 1,
                end_line: find_ts_block_end(&lines, i),
                visibility: if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                ..Default::default()
            });
            continue;
        }

        if in_class {
            if let Some(cap) = re_method.captures(line) {
                let vis_str = cap.get(1).map_or("public", |m| m.as_str());
                let name = cap[2].to_string();
                let keywords = [
                    "if", "for", "while", "switch", "catch", "return", "throw", "new", "typeof",
                    "constructor",
                ];
                if !keywords.contains(&name.as_str()) {
                    let vis = match vis_str {
                        "private" => Visibility::Private,
                        "protected" => Visibility::Protected,
                        _ => Visibility::Public,
                    };
                    let kind = if name == "constructor" {
                        SymbolKind::Method
                    } else {
                        SymbolKind::Method
                    };
                    let params = extract_ts_params(line);
                    symbols.push(IndexedSymbol {
                        name,
                        kind,
                        line: i + 1,
                        end_line: find_ts_block_end(&lines, i),
                        visibility: vis,
                        params,
                        ..Default::default()
                    });
                }
            }
        }
    }

    (symbols, imports)
}

fn extract_ts_imports(content: &str) -> Vec<IndexedImport> {
    let mut imports = Vec::new();
    let re_import =
        Regex::new(r#"import\s+(?:(?:\{([^}]*)\})|(\w+))\s+from\s+['"]([^'"]+)['"]"#).unwrap();
    let re_require = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();

    for cap in re_import.captures_iter(content) {
        let module = cap.get(3).map_or("", |m| m.as_str());
        let symbols = if let Some(names) = cap.get(1) {
            names
                .as_str()
                .split(',')
                .map(|s| {
                    let s = s.trim();
                    s.split(" as ").next().unwrap_or(s).trim().to_string()
                })
                .filter(|s| !s.is_empty())
                .collect()
        } else if let Some(default_name) = cap.get(2) {
            vec![default_name.as_str().to_string()]
        } else {
            Vec::new()
        };
        imports.push(IndexedImport {
            raw: module.to_string(),
            symbols,
            ..Default::default()
        });
    }

    for cap in re_require.captures_iter(content) {
        imports.push(IndexedImport {
            raw: cap[1].to_string(),
            symbols: Vec::new(),
            ..Default::default()
        });
    }

    imports
}

fn extract_ts_params(line: &str) -> Vec<Parameter> {
    let re_params = Regex::new(r"\(([^)]*)\)").unwrap();
    let cap = match re_params.captures(line) {
        Some(c) => c,
        None => return Vec::new(),
    };
    cap[1]
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() {
                return None;
            }
            let p = p.trim_start_matches("...");
            if let Some((name, type_hint)) = p.split_once(':') {
                let name = name.trim().to_string();
                if name.is_empty() {
                    return None;
                }
                Some(Parameter {
                    name,
                    type_hint: Some(type_hint.trim().to_string()),
                })
            } else {
                let name = p.split('=').next().unwrap_or(p).trim().to_string();
                if name.is_empty() {
                    return None;
                }
                Some(Parameter {
                    name,
                    type_hint: None,
                })
            }
        })
        .collect()
}

fn find_ts_block_end(lines: &[&str], start: usize) -> usize {
    find_rust_block_end(lines, start, 0)
}

// --- Go ---

fn extract_go(content: &str) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    let mut symbols = Vec::new();
    let imports = extract_go_imports(content);
    let lines: Vec<&str> = content.lines().collect();

    let re_func = Regex::new(r"^func\s+(?:\((\w+)\s+\*?(\w+)\)\s+)?(\w+)\s*\(").unwrap();
    let re_struct = Regex::new(r"^type\s+(\w+)\s+struct").unwrap();
    let re_interface = Regex::new(r"^type\s+(\w+)\s+interface").unwrap();
    let _re_const = Regex::new(r"^\s*(\w+)\s*=").unwrap();
    let re_type_alias = Regex::new(r"^type\s+(\w+)\s+(\w+)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(cap) = re_func.captures(line) {
            let receiver_type = cap.get(2).map(|m| m.as_str().to_string());
            let name = cap[3].to_string();
            let kind = if receiver_type.is_some() {
                SymbolKind::Method
            } else {
                SymbolKind::Function
            };
            let vis = if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let params = extract_go_params(line);
            let end = find_rust_block_end(&lines, i, 0);
            symbols.push(IndexedSymbol {
                name,
                kind,
                line: i + 1,
                end_line: end,
                visibility: vis,
                params,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_struct.captures(line) {
            let name = cap[1].to_string();
            let vis = if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let end = find_rust_block_end(&lines, i, 0);
            let fields = extract_go_struct_fields(&lines, i);
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Struct,
                line: i + 1,
                end_line: end,
                visibility: vis,
                fields,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_interface.captures(line) {
            let name = cap[1].to_string();
            let vis = if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                Visibility::Public
            } else {
                Visibility::Private
            };
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Interface,
                line: i + 1,
                end_line: find_rust_block_end(&lines, i, 0),
                visibility: vis,
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_type_alias.captures(line) {
            let name = cap[1].to_string();
            let target = cap[2].to_string();
            if target != "struct" && target != "interface" {
                let vis = if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                    Visibility::Public
                } else {
                    Visibility::Private
                };
                symbols.push(IndexedSymbol {
                    name,
                    kind: SymbolKind::TypeAlias,
                    line: i + 1,
                    end_line: i + 1,
                    visibility: vis,
                    ..Default::default()
                });
            }
        }
    }

    (symbols, imports)
}

fn extract_go_imports(content: &str) -> Vec<IndexedImport> {
    let mut imports = Vec::new();
    let re_single = Regex::new(r#"^\s*import\s+"([^"]+)"#).unwrap();
    let re_block = Regex::new(r#"(?s)import\s*\((.*?)\)"#).unwrap();
    let re_quoted = Regex::new(r#""([^"]+)""#).unwrap();

    for cap in re_single.captures_iter(content) {
        imports.push(IndexedImport {
            raw: cap[1].to_string(),
            symbols: Vec::new(),
            ..Default::default()
        });
    }

    for cap in re_block.captures_iter(content) {
        let block = &cap[1];
        for qcap in re_quoted.captures_iter(block) {
            imports.push(IndexedImport {
                raw: qcap[1].to_string(),
                symbols: Vec::new(),
                ..Default::default()
            });
        }
    }

    imports
}

fn extract_go_params(line: &str) -> Vec<Parameter> {
    let re_params = Regex::new(r"\(([^)]*)\)").unwrap();
    let cap = match re_params.captures(line) {
        Some(c) => c,
        None => return Vec::new(),
    };
    cap[1]
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() {
                return None;
            }
            let parts: Vec<&str> = p.split_whitespace().collect();
            match parts.len() {
                0 => None,
                1 => Some(Parameter {
                    name: parts[0].to_string(),
                    type_hint: None,
                }),
                _ => Some(Parameter {
                    name: parts[0].to_string(),
                    type_hint: Some(parts[1..].join(" ")),
                }),
            }
        })
        .collect()
}

fn extract_go_struct_fields(lines: &[&str], struct_line: usize) -> Vec<String> {
    let mut fields = Vec::new();
    let re_field = Regex::new(r"^\s+(\w+)\s+").unwrap();

    for i in (struct_line + 1)..lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "}" || trimmed.starts_with('}') {
            break;
        }
        if let Some(cap) = re_field.captures(line) {
            fields.push(cap[1].to_string());
        }
    }

    fields
}

// --- Java ---

fn extract_java(content: &str) -> (Vec<IndexedSymbol>, Vec<IndexedImport>) {
    let mut symbols = Vec::new();
    let imports = extract_java_imports(content);
    let lines: Vec<&str> = content.lines().collect();

    let re_class = Regex::new(
        r"^(\s*)(public|private|protected)?\s*(?:abstract\s+)?(?:static\s+)?(?:final\s+)?class\s+(\w+)",
    )
    .unwrap();
    let re_interface =
        Regex::new(r"^(\s*)(public|private|protected)?\s*interface\s+(\w+)").unwrap();
    let re_enum =
        Regex::new(r"^(\s*)(public|private|protected)?\s*enum\s+(\w+)").unwrap();
    let re_method = Regex::new(
        r"^\s+(public|private|protected)?\s*(?:static\s+)?(?:final\s+)?(?:synchronized\s+)?(?:\w+(?:<[^>]*>)?(?:\[\])*)\s+(\w+)\s*\(",
    )
    .unwrap();
    let re_const = Regex::new(
        r"^\s+(public|private|protected)?\s*(?:static\s+)?(?:final\s+)?\w+\s+(\w+)\s*=",
    )
    .unwrap();

    let mut in_class = false;
    let mut _class_indent: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }

        if let Some(cap) = re_class.captures(line) {
            let indent = cap.get(1).map_or(0, |m| m.as_str().len());
            let vis_str = cap.get(2).map_or("public", |m| m.as_str());
            let name = cap[3].to_string();
            in_class = true;
            _class_indent = indent;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Class,
                line: i + 1,
                end_line: find_rust_block_end(&lines, i, indent),
                visibility: java_vis(vis_str),
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_interface.captures(line) {
            let vis_str = cap.get(2).map_or("public", |m| m.as_str());
            let name = cap[3].to_string();
            in_class = false;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Interface,
                line: i + 1,
                end_line: find_rust_block_end(&lines, i, 0),
                visibility: java_vis(vis_str),
                ..Default::default()
            });
            continue;
        }

        if let Some(cap) = re_enum.captures(line) {
            let vis_str = cap.get(2).map_or("public", |m| m.as_str());
            let name = cap[3].to_string();
            in_class = false;
            symbols.push(IndexedSymbol {
                name,
                kind: SymbolKind::Enum,
                line: i + 1,
                end_line: find_rust_block_end(&lines, i, 0),
                visibility: java_vis(vis_str),
                ..Default::default()
            });
            continue;
        }

        if in_class {
            if let Some(cap) = re_method.captures(line) {
                let vis_str = cap.get(1).map(|m| m.as_str()).unwrap_or("public");
                let name = cap[2].to_string();
                let keywords = ["if", "for", "while", "switch", "catch", "return", "new"];
                if !keywords.contains(&name.as_str()) {
                    let params = extract_ts_params(line);
                    symbols.push(IndexedSymbol {
                        name,
                        kind: SymbolKind::Method,
                        line: i + 1,
                        end_line: find_rust_block_end(&lines, i, 0),
                        visibility: java_vis(vis_str),
                        params,
                        ..Default::default()
                    });
                }
                continue;
            }

            if let Some(cap) = re_const.captures(line) {
                let vis_str = cap.get(1).map(|m| m.as_str()).unwrap_or("public");
                let name = cap[2].to_string();
                symbols.push(IndexedSymbol {
                    name,
                    kind: SymbolKind::Constant,
                    line: i + 1,
                    end_line: i + 1,
                    visibility: java_vis(vis_str),
                    ..Default::default()
                });
            }
        }
    }

    (symbols, imports)
}

fn extract_java_imports(content: &str) -> Vec<IndexedImport> {
    let re_import = Regex::new(r"(?m)^\s*import\s+(?:static\s+)?([\w.]+);").unwrap();
    re_import
        .captures_iter(content)
        .map(|cap| IndexedImport {
            raw: cap[1].to_string(),
            symbols: Vec::new(),
            ..Default::default()
        })
        .collect()
}

fn java_vis(s: &str) -> Visibility {
    match s {
        "private" => Visibility::Private,
        "protected" => Visibility::Protected,
        _ => Visibility::Public,
    }
}
