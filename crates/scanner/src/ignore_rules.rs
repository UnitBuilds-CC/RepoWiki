use std::path::Path;

pub struct IgnoreRules {
    patterns: Vec<(String, bool)>,
}

impl IgnoreRules {
    pub fn from_root(root: &Path) -> Self {
        let mut patterns = Vec::new();
        for name in &[".gitignore", ".repowikiignore"] {
            let path = root.join(name);
            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };
            for raw_line in content.lines() {
                let line = raw_line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let (pat, negated) = if let Some(rest) = line.strip_prefix('!') {
                    (rest.trim(), true)
                } else {
                    (line, false)
                };
                if !pat.is_empty() {
                    patterns.push((pat.replace('\\', "/"), negated));
                }
            }
        }
        Self { patterns }
    }

    pub fn matches(&self, rel_path: &str, is_dir: bool) -> bool {
        let rel_path = rel_path.replace('\\', "/");
        let rel_path = rel_path.trim_matches('/');
        let mut ignored = false;
        for (pattern, negated) in &self.patterns {
            if matches_ignore_pattern(pattern, rel_path, is_dir) {
                ignored = !negated;
            }
        }
        ignored
    }
}

fn matches_ignore_pattern(pattern: &str, rel_path: &str, is_dir: bool) -> bool {
    let dir_pattern = pattern.ends_with('/');
    let pattern = pattern.trim_matches('/');
    if pattern.is_empty() {
        return false;
    }
    if dir_pattern {
        return is_dir
            && (rel_path == pattern || rel_path.starts_with(&format!("{pattern}/")));
    }
    if pattern.contains('/') {
        if glob_match(pattern, rel_path) {
            return true;
        }
        let prefix = pattern.trim_end_matches('*');
        return rel_path.starts_with(&format!("{prefix}/"));
    }
    rel_path.split('/').any(|part| glob_match(pattern, part))
}

fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), text.as_bytes())
}

fn glob_match_inner(mut pat: &[u8], mut text: &[u8]) -> bool {
    let mut star_pat: Option<&[u8]> = None;
    let mut star_text: Option<usize> = None;

    let mut ti = 0usize;
    let mut pi = 0usize;

    while ti < text.len() {
        if pi < pat.len() && (pat[pi] == b'?' || pat[pi] == text[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < pat.len() && pat[pi] == b'*' {
            star_pat = Some(&pat[pi..]);
            star_text = Some(ti);
            pi += 1;
        } else if let (Some(sp), Some(st)) = (star_pat, star_text) {
            pat = sp;
            pi = 0;
            let new_st = st + 1;
            text = &text[st..];
            ti = 0;
            star_text = Some(new_st);
            // skip current char
            if ti >= text.len() {
                break;
            }
            ti += 1;
        } else {
            return false;
        }
    }

    while pi < pat.len() && pat[pi] == b'*' {
        pi += 1;
    }
    pi == pat.len()
}
