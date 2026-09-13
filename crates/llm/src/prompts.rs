use crate::client::ChatMessage;
use regex::Regex;

fn lang_instruction(language: &str) -> &'static str {
    match language {
        "zh" => "请用中文回答。",
        "ja" => "日本語で回答してください。",
        "ko" => "한국어로 답변해주세요.",
        _ => "Respond in English.",
    }
}

fn json_instruction() -> &'static str {
    "Output ONLY valid JSON. No markdown fences, no explanation text before or after. Just the JSON object/array."
}

pub fn build_overview_prompt(file_tree: &str, key_files: &str, language: &str) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: "system".into(),
            content: format!(
                "You are a senior software engineer explaining a project to a new team member. \
                 Be direct, specific, and concrete. \
                 Do NOT use filler phrases like 'leveraging', 'utilizing', 'cutting-edge', \
                 'robust', or 'comprehensive'. Just describe what things do. \
                 {}",
                lang_instruction(language)
            ),
        },
        ChatMessage {
            role: "user".into(),
            content: format!(
                "Here is the file tree and key files of a project:\n\n\
                 ## File Tree\n```\n{file_tree}\n```\n\n\
                 ## Key Files\n{key_files}\n\n\
                 Generate a project overview as JSON with this structure:\n\
                 {{\n\
                 \x20 \"name\": \"project name\",\n\
                 \x20 \"one_liner\": \"what this project does in one sentence (max 20 words)\",\n\
                 \x20 \"description\": \"2-3 paragraphs explaining the project in plain language\",\n\
                 \x20 \"tech_stack\": [{{\"name\": \"Python\", \"category\": \"language\", \"version\": \"3.10+\"}}],\n\
                 \x20 \"setup_instructions\": [\"step 1\", \"step 2\"],\n\
                 \x20 \"key_features\": [\"feature 1\", \"feature 2\"]\n\
                 }}\n\n\
                 {}",
                json_instruction()
            ),
        },
    ]
}

pub fn build_module_prompt(
    module_name: &str,
    files_context: &str,
    project_summary: &str,
    language: &str,
) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: "system".into(),
            content: format!(
                "You are a senior engineer documenting your own code. \
                 Be direct and specific. No filler. \
                 Explain what each file does, how files relate to each other, \
                 and what the key functions/classes are. \
                 {}",
                lang_instruction(language)
            ),
        },
        ChatMessage {
            role: "user".into(),
            content: format!(
                "Project: {project_summary}\n\n\
                 Document the '{module_name}' module. Here are its files:\n\n\
                 {files_context}\n\n\
                 Output JSON:\n\
                 {{\n\
                 \x20 \"name\": \"{module_name}\",\n\
                 \x20 \"purpose\": \"one sentence\",\n\
                 \x20 \"description\": \"detailed explanation\",\n\
                 \x20 \"files\": [\n\
                 \x20   {{\"path\": \"file.py\", \"purpose\": \"what it does\", \
                 \"key_symbols\": [{{\"name\": \"func_name\", \"kind\": \"function\", \"description\": \"...\"}}]}}\n\
                 \x20 ],\n\
                 \x20 \"relationships\": [{{\"source\": \"a.py\", \"target\": \"b.py\", \"description\": \"a imports b for...\"}}],\n\
                 \x20 \"key_concepts\": [{{\"name\": \"concept\", \"explanation\": \"...\"}}]\n\
                 }}\n\n\
                 {}",
                json_instruction()
            ),
        },
    ]
}

pub fn build_module_prompt_from_index(
    module_name: &str,
    index_context: &str,
    project_summary: &str,
    language: &str,
) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: "system".into(),
            content: format!(
                "You are a senior engineer documenting your own code. \
                 Be direct and specific. No filler. \
                 The symbols, imports, and relationships below were extracted automatically. \
                 Your job is to describe what they DO and WHY they exist, not repeat the structure. \
                 Explain the purpose of each file, how files relate to each other, \
                 and what the key functions/classes accomplish. \
                 {}",
                lang_instruction(language)
            ),
        },
        ChatMessage {
            role: "user".into(),
            content: format!(
                "Project: {project_summary}\n\n\
                 Document the '{module_name}' module.\n\n\
                 {index_context}\n\n\
                 Output JSON:\n\
                 {{\n\
                 \x20 \"name\": \"{module_name}\",\n\
                 \x20 \"purpose\": \"one sentence\",\n\
                 \x20 \"description\": \"detailed explanation of what this module does and why it exists\",\n\
                 \x20 \"files\": [\n\
                 \x20   {{\"path\": \"file.py\", \"purpose\": \"what it does and why\", \
                 \"key_symbols\": [{{\"name\": \"func_name\", \"kind\": \"function\", \"description\": \"what it does\"}}]}}\n\
                 \x20 ],\n\
                 \x20 \"relationships\": [{{\"source\": \"a.py\", \"target\": \"b.py\", \"description\": \"how they interact\"}}],\n\
                 \x20 \"key_concepts\": [{{\"name\": \"concept\", \"explanation\": \"why it matters\"}}]\n\
                 }}\n\n\
                 {}",
                json_instruction()
            ),
        },
    ]
}

pub fn build_architecture_prompt(
    file_tree: &str,
    key_files: &str,
    module_summary: &str,
    language: &str,
) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: "system".into(),
            content: format!(
                "You are a software architect analyzing a codebase. \
                 Identify the architecture pattern and generate Mermaid diagrams. \
                 Mermaid syntax must be valid. Use simple node names (no special chars). \
                 Include ALL modules/components listed below in your component diagram. \
                 {}",
                lang_instruction(language)
            ),
        },
        ChatMessage {
            role: "user".into(),
            content: format!(
                "## File Tree\n```\n{file_tree}\n```\n\n\
                 ## Modules\n{module_summary}\n\n\
                 ## Key Files\n{key_files}\n\n\
                 Analyze the architecture. Output JSON:\n\
                 {{\n\
                 \x20 \"architecture_type\": \"one of: monolith, client-server, microservices, library, cli-tool, framework, plugin-system, pipeline\",\n\
                 \x20 \"description\": \"explain the architecture in 2-3 sentences\",\n\
                 \x20 \"components\": [{{\"name\": \"...\", \"purpose\": \"...\", \"files\": [\"...\"]}}],\n\
                 \x20 \"mermaid_component\": \"graph TD\\n  A[Component] --> B[Component]\\n  ...\",\n\
                 \x20 \"mermaid_sequence\": \"sequenceDiagram\\n  participant A\\n  A->>B: request\\n  ...\",\n\
                 \x20 \"data_flow\": \"describe the main data flow in 2-3 sentences\"\n\
                 }}\n\n\
                 IMPORTANT: The components array MUST include every module listed above. \
                 Mermaid code must be a single string with \\n for newlines. \
                 Use simple alphanumeric node IDs. \
                 {}",
                json_instruction()
            ),
        },
    ]
}

pub fn build_reading_guide_prompt(
    rankings: &str,
    module_summaries: &str,
    language: &str,
) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: "system".into(),
            content: format!(
                "You are a mentor helping a developer understand a new codebase. \
                 Create a reading guide: which files to read, in what order, and why. \
                 Start from entry points and configuration, then core logic, then utilities. \
                 Each step should say WHAT to look for, not just WHICH files. \
                 {}",
                lang_instruction(language)
            ),
        },
        ChatMessage {
            role: "user".into(),
            content: format!(
                "## File Importance Rankings (by PageRank)\n{rankings}\n\n\
                 ## Module Summaries\n{module_summaries}\n\n\
                 Create a reading guide with 5-10 steps. Output JSON:\n\
                 {{\n\
                 \x20 \"introduction\": \"brief intro on how to approach this codebase\",\n\
                 \x20 \"steps\": [\n\
                 \x20   {{\"order\": 1, \"title\": \"step title\", \"files\": [\"file1.py\", \"file2.py\"], \
                 \"explanation\": \"what to look for and why\", \"time_estimate\": \"5 min\"}}\n\
                 \x20 ],\n\
                 \x20 \"tips\": [\"general tip 1\", \"general tip 2\"]\n\
                 }}\n\n\
                 {}",
                json_instruction()
            ),
        },
    ]
}

pub fn build_chat_prompt(
    question: &str,
    context_chunks: &str,
    language: &str,
    history: &[ChatMessage],
) -> Vec<ChatMessage> {
    let mut messages = vec![ChatMessage {
        role: "system".into(),
        content: format!(
            "You are a knowledgeable developer answering questions about a codebase. \
             Answer based on the actual code shown below, not general knowledge. \
             Reference specific files and line numbers when relevant. \
             Be direct: answer the question, don't give a lecture. \
             {}",
            lang_instruction(language)
        ),
    }];

    let max_turns = 6 * 2;
    let start = if history.len() > max_turns {
        history.len() - max_turns
    } else {
        0
    };
    for turn in &history[start..] {
        if (turn.role == "user" || turn.role == "assistant") && !turn.content.trim().is_empty() {
            messages.push(ChatMessage {
                role: turn.role.clone(),
                content: turn.content.trim().to_string(),
            });
        }
    }

    messages.push(ChatMessage {
        role: "user".into(),
        content: format!("## Relevant Code\n{context_chunks}\n\n## Question\n{question}"),
    });

    messages
}

pub fn extract_json(text: &str) -> Option<serde_json::Value> {
    let re_fence_start = Regex::new(r"^```(?:json)?\s*\n?").ok()?;
    let re_fence_end = Regex::new(r"\n?```\s*$").ok()?;

    let text = re_fence_start.replace(text.trim(), "");
    let text = re_fence_end.replace(text.trim(), "");

    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
        return Some(v);
    }

    let mut best_match: Option<serde_json::Value> = None;
    let mut best_size = 0;

    for (start_char, end_char) in [('{', '}'), ('[', ']')] {
        let mut start_search = 0;
        while let Some(start) = text[start_search..].find(start_char) {
            let start = start_search + start;
            if let Some(end) = text[start..].rfind(end_char) {
                let end = start + end;
                if end > start {
                    let candidate = &text[start..=end];
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(candidate) {
                        if candidate.len() > best_size {
                            best_size = candidate.len();
                            best_match = Some(v);
                        }
                    }
                }
            }
            start_search = start + 1;
        }
    }

    best_match
}
