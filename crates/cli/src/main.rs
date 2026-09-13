use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use console::{style, Term};

use repowiki_core::config::{resolve_model, Config};
use repowiki_core::models::{ProjectContext, ScanReport};
use repowiki_graph::DependencyGraph;
use repowiki_ingest::parse_git_url;
use repowiki_scanner::scan_directory;

#[derive(Parser)]
#[command(name = "repowiki", version, about = "Generate wiki documentation for any codebase")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the repo map: files ranked by dependency PageRank
    Map {
        /// Path to the repository
        path: String,
        /// Max entries to show
        #[arg(short = 'n', long, default_value = "50")]
        top: usize,
        /// Output format
        #[arg(long, default_value = "text")]
        format: MapFormat,
    },
    /// Scan a local directory or GitHub URL and generate wiki documentation
    Scan {
        /// Path or URL to scan
        path_or_url: String,
        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
        /// Output format
        #[arg(short, long, default_value = "markdown")]
        format: ScanFormat,
        /// Output language (en/zh/ja/ko)
        #[arg(short = 'l', long)]
        lang: Option<String>,
        /// LLM model name or alias
        #[arg(short, long)]
        model: Option<String>,
        /// Open HTML output in browser
        #[arg(long)]
        open: bool,
        /// Ignore incremental state and regenerate every page
        #[arg(long)]
        full: bool,
        /// Write GitHub Pages loader (index.html + .nojekyll)
        #[arg(long)]
        site: bool,
    },
    /// Start the RepoWiki web interface
    Serve {
        /// Path or URL to preload
        #[arg(default_value = ".")]
        path_or_url: String,
        /// Port to serve on
        #[arg(short, long, default_value = "8000")]
        port: u16,
    },
    /// Wipe cached LLM analysis results
    CacheClear,
    /// Dump the symbol index for a repository as JSON
    Index {
        /// Path to the repository
        path: String,
    },
    /// Ask questions about a codebase in the terminal
    Chat {
        /// Path or URL to chat about
        #[arg(default_value = ".")]
        path_or_url: String,
        /// LLM model name or alias
        #[arg(short, long)]
        model: Option<String>,
        /// Answer language (en/zh/ja/ko)
        #[arg(short = 'l', long)]
        lang: Option<String>,
    },
    /// Manage RepoWiki configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Clone, ValueEnum)]
enum MapFormat {
    Text,
    Json,
}

#[derive(Clone, ValueEnum)]
enum ScanFormat {
    Markdown,
    Json,
    Html,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a config value
    Set { key: String, value: String },
    /// Get a config value
    Get { key: String },
    /// Show all config values
    List,
}

fn is_url(s: &str) -> bool {
    s.starts_with("http") || parse_git_url(s).is_some()
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let term = Term::stdout();

    match cli.command {
        Commands::Map { path, top, format } => cmd_map(&path, top, format, &term)?,
        Commands::Scan {
            path_or_url,
            output,
            format,
            lang,
            model,
            open,
            full,
            site,
        } => {
            cmd_scan(
                &path_or_url, output, format, lang, model, open, full, site, &term,
            )
            .await?
        }
        Commands::Serve { path_or_url, port } => cmd_serve(&path_or_url, port).await?,
        Commands::CacheClear => cmd_cache_clear().await?,
        Commands::Index { path } => cmd_index(&path).await?,
        Commands::Chat {
            path_or_url,
            model,
            lang,
        } => cmd_chat(&path_or_url, model, lang, &term).await?,
        Commands::Config { action } => cmd_config(action)?,
    }

    Ok(())
}

fn cmd_map(path: &str, top: usize, format: MapFormat, term: &Term) -> Result<()> {
    if is_url(path) {
        bail!("map works on local directories only, not URLs");
    }
    if top == 0 {
        bail!("--top must be greater than zero");
    }

    let path = Path::new(path);
    let mut report = ScanReport::default();
    let files = scan_directory(path, 200 * 1024, 1000, 2, Some(&mut report), &[])?;
    let project = ProjectContext {
        name: path.to_string_lossy().into_owned(),
        root: path.to_string_lossy().into_owned(),
        files: files.clone(),
        file_tree: String::new(),
        coverage: Some(report.clone()),
    };
    let ranked = DependencyGraph::build_from_project(&project).rank_files();

    if report.partial() {
        term.write_line(&format!("Partial coverage: {}", report.summary_line()))?;
    }

    let unknown = String::from("unknown");
    let entries: Vec<_> = ranked
        .iter()
        .take(top)
        .enumerate()
        .map(|(i, (p, score))| {
            let file = files.iter().find(|f| f.path == *p);
            serde_json::json!({
                "rank": i + 1,
                "path": p.replace('\\', "/"),
                "score": (*score * 1e6).round() / 1e6,
                "language": file.map(|f| &f.language).unwrap_or(&unknown),
                "lines": file.map(|f| f.lines).unwrap_or(0),
            })
        })
        .collect();

    match format {
        MapFormat::Json => {
            let payload = serde_json::json!({
                "root": path.to_string_lossy(),
                "file_count": files.len(),
                "entries": entries,
                "partial_coverage": if report.partial() { Some(report.summary_line()) } else { None },
            });
            term.write_line(&serde_json::to_string_pretty(&payload)?)?;
        }
        MapFormat::Text => {
            println!(
                "\n{}",
                style(format!("Repo map: {} ({} files)", path.display(), files.len())).bold()
            );
            println!(
                "{:>5}  {:>8}  {:<50}  {:<10}  {:>7}",
                "#", "Score", "Path", "Lang", "Lines"
            );
            println!("{}", "-".repeat(85));

            let peak = entries.first().and_then(|e| e["score"].as_f64()).unwrap_or(1.0);
            for e in &entries {
                let rel = e["score"].as_f64().unwrap_or(0.0) / peak;
                println!(
                    "{:>5}  {:>8.3}  {:<50}  {:<10}  {:>7}",
                    e["rank"].as_u64().unwrap_or(0),
                    rel,
                    e["path"].as_str().unwrap_or(""),
                    e["language"].as_str().unwrap_or("unknown"),
                    e["lines"].as_u64().unwrap_or(0),
                );
            }
            println!(
                "\n{}",
                style("Scores are PageRank over the real import graph, normalized to the top file.")
                    .dim()
            );
        }
    }

    Ok(())
}

async fn cmd_scan(
    path_or_url: &str,
    output: Option<String>,
    format: ScanFormat,
    lang: Option<String>,
    model: Option<String>,
    open_browser: bool,
    full: bool,
    site: bool,
    term: &Term,
) -> Result<()> {
    let mut cfg = Config::load();
    if let Some(l) = lang {
        cfg.language = l;
    }
    if let Some(m) = model {
        cfg.model = resolve_model(&m);
    }
    if let Some(o) = output {
        cfg.output_dir = o;
    }

    let exclude_dirs: Vec<String> = {
        let p = Path::new(&cfg.output_dir);
        p.components()
            .filter_map(|c| c.as_os_str().to_str())
            .filter(|s| *s != "." && *s != "..")
            .take(1)
            .map(|s| s.to_string())
            .collect()
    };

    let project = if is_url(path_or_url) {
        repowiki_ingest::ingest_github(path_or_url, cfg.max_file_size, cfg.max_files, false, &exclude_dirs)
            .map_err(|e| anyhow::anyhow!(e))?
    } else {
        repowiki_ingest::ingest_local(path_or_url, cfg.max_file_size, cfg.max_files, &exclude_dirs)?
    };

    println!();
    println!("{} {}", style("Project:").bold().green(), project.name);
    println!("{} {}", style("Files:").bold().green(), project.files.len());
    println!("{} {}", style("Lines:").bold().green(), project.total_lines());

    let mut lang_counts: HashMap<String, usize> = HashMap::new();
    for f in &project.files {
        *lang_counts.entry(f.language.clone()).or_default() += 1;
    }

    if !lang_counts.is_empty() {
        println!("\n{}", style("Languages").bold());
        let mut sorted: Vec<_> = lang_counts.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (language, count) in sorted.iter().take(10) {
            println!("  {:<20} {}", style(language).cyan(), count);
        }
    }

    if cfg.api_key.is_empty() {
        println!();
        println!(
            "{}",
            style("No API key configured. Showing scan results only.").yellow()
        );
        println!("Set one with: {}", style("repowiki config set api_key YOUR_KEY").bold());
        println!("Or set DEEPSEEK_API_KEY / OPENAI_API_KEY / ANTHROPIC_API_KEY env var.");
        return Ok(());
    }

    run_analysis(&project, &cfg, format, open_browser, site, full, term).await
}

async fn run_analysis(
    project: &ProjectContext,
    cfg: &Config,
    format: ScanFormat,
    open_browser: bool,
    site: bool,
    full: bool,
    term: &Term,
) -> Result<()> {
    use repowiki_analyzer::Analyzer;
    use repowiki_cache::Cache;
    use repowiki_llm::client::LLMClient;
    use repowiki_wiki::WikiBuilder;

    let llm = LLMClient::new(&cfg.model, &cfg.api_key, &cfg.api_base);
    let cache = Cache::open(None)?;
    let mut analyzer = Analyzer::new(llm.clone(), cache.clone(), &cfg.language, cfg.concurrency as usize);

    let progress = |msg: &str| {
        println!("{} {}", style("→").cyan(), msg);
    };

    let wiki_data = analyzer.analyze(project, Some(Arc::new(progress))).await;

    let graph = DependencyGraph::build_from_project(project);
    let builder = WikiBuilder;
    let wiki = builder.build(project, &wiki_data, &graph);

    let output_dir = &cfg.output_dir;

    match format {
        ScanFormat::Markdown => {
            let summary = repowiki_export::export_markdown(
                &wiki,
                Path::new(output_dir),
                Some(&analyzer.cache_keys),
                &cfg.model,
                &cfg.language,
                full,
            );
            if let Some(ref s) = summary {
                if !s.kept.is_empty() || !s.removed.is_empty() {
                    println!(
                        "{}",
                        style(format!(
                            "Incremental: {} pages written, {} unchanged, {} removed",
                            s.written.len(),
                            s.kept.len(),
                            s.removed.len()
                        ))
                        .dim()
                    );
                }
            }
            if site {
                repowiki_export::write_site_loader(output_dir, &wiki.project_name)?;
                println!("{}", style("GitHub Pages loader written (index.html + .nojekyll)").dim());
            }
            println!("\n{} {}/", style("Wiki generated:").bold().green(), output_dir);
        }
        ScanFormat::Json => {
            let out_path = format!("{}/repowiki.json", output_dir);
            let written = repowiki_export::export_json(&wiki, Path::new(&out_path));
            println!("\n{} {}", style("Wiki generated:").bold().green(), out_path);
            if !written {
                println!("{}", style("Incremental: content unchanged, file left as is").dim());
            }
        }
        ScanFormat::Html => {
            let out_path = format!("{}/repowiki.html", output_dir);
            let written = repowiki_export::export_html(&wiki, Path::new(&out_path));
            println!("\n{} {}", style("Wiki generated:").bold().green(), out_path);
            if !written {
                println!("{}", style("Incremental: content unchanged, file left as is").dim());
            }
            if open_browser {
                println!(
                    "{}",
                    style(format!("Open in browser: file://{}", Path::new(&out_path).canonicalize().unwrap_or_default().display())).dim()
                );
            }
        }
    }

    let _ = term;

    let input_tokens = llm.total_input_tokens();
    let output_tokens = llm.total_output_tokens();
    if input_tokens > 0 || output_tokens > 0 {
        let cost = llm.total_cost();
        let cost_str = if cost > 0.0 {
            format!(" (${:.4})", cost)
        } else {
            String::new()
        };
        println!(
            "{}",
            style(format!(
                "Tokens used: {} in / {} out{}",
                input_tokens, output_tokens, cost_str
            ))
            .dim()
        );
    }

    Ok(())
}

async fn cmd_serve(path_or_url: &str, port: u16) -> Result<()> {
    use repowiki_cache::Cache;
    use repowiki_server::{AppState, create_app};
    use tokio::net::TcpListener;

    println!(
        "{}",
        style(format!("Starting RepoWiki server on port {}...", port)).cyan().bold()
    );
    println!("{} http://localhost:{}", style("Open:").bold(), port);

    if path_or_url != "." {
        std::env::set_var("REPOWIKI_SERVE_TARGET", path_or_url);
    }

    let cache = Cache::open(None)?;
    let state = AppState::new(cache);
    let app = create_app(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn cmd_cache_clear() -> Result<()> {
    use repowiki_cache::Cache;

    let cache = Cache::open(None)?;
    let count = cache.clear()?;
    println!(
        "Cleared {} cached analysis entr{}.",
        count,
        if count == 1 { "y" } else { "ies" }
    );
    Ok(())
}

async fn cmd_index(path: &str) -> Result<()> {
    use repowiki_cache::Cache;
    use repowiki_index::build_index;

    if is_url(path) {
        bail!("index works on local directories only, not URLs");
    }

    let cfg = Config::load();
    let p = Path::new(path);
    let files = repowiki_ingest::ingest_local(path, cfg.max_file_size, cfg.max_files, &[])?;

    println!(
        "{} {} ({} files)",
        style("Indexing:").bold().green(),
        files.name,
        files.files.len()
    );

    let cache = Cache::open(None)?;
    let key_prefix = format!("{}:{}", cfg.model, cfg.language);
    let index = build_index(&files.files, &cache, &key_prefix);

    let total_symbols: usize = index.modules.iter().map(|m| m.total_symbols).sum();
    let total_edges = index.call_graph.len();

    println!(
        "{} {} modules, {} symbols, {} call edges",
        style("Index:").bold().green(),
        index.modules.len(),
        total_symbols,
        total_edges
    );

    let json = serde_json::to_string_pretty(&index)?;
    println!("\n{}", json);

    let _ = p;
    Ok(())
}

async fn cmd_chat(
    path_or_url: &str,
    model: Option<String>,
    lang: Option<String>,
    term: &Term,
) -> Result<()> {
    use repowiki_llm::client::{ChatMessage, LLMClient};
    use repowiki_llm::prompts::build_chat_prompt;
    use repowiki_rag::{format_context, load_or_build_index};

    let mut cfg = Config::load();
    if let Some(m) = model {
        cfg.model = resolve_model(&m);
    }
    if let Some(l) = lang {
        cfg.language = l;
    }

    if cfg.api_key.is_empty() {
        println!("{}", style("No API key configured.").yellow());
        println!("Chat needs an LLM.");
        println!("Set one with: {}", style("repowiki config set api_key YOUR_KEY").bold());
        println!("Or set DEEPSEEK_API_KEY / OPENAI_API_KEY / ANTHROPIC_API_KEY env var.");
        return Ok(());
    }

    println!("{}", style("Indexing repository...").dim());

    let project = if is_url(path_or_url) {
        repowiki_ingest::ingest_github(path_or_url, cfg.max_file_size, cfg.max_files, false, &[])
            .map_err(|e| anyhow::anyhow!(e))?
    } else {
        repowiki_ingest::ingest_local(path_or_url, cfg.max_file_size, cfg.max_files, &[])?
    };

    let (rag, index_cached) = load_or_build_index(&project, None);
    if index_cached {
        println!(
            "{}",
            style(format!("Index unchanged, loaded from cache ({} chunks).", rag.chunks.len()))
                .dim()
        );
    }
    if rag.chunks.is_empty() {
        println!("{}", style("No readable source found to chat about.").yellow());
        return Ok(());
    }

    println!(
        "\n{} {} ({} files). Remembers this session. Type 'exit' to quit.\n",
        style("RepoWiki Chat:").bold().cyan(),
        project.name,
        project.files.len()
    );

    let llm = LLMClient::new(&cfg.model, &cfg.api_key, &cfg.api_base);
    let mut history: Vec<ChatMessage> = Vec::new();

    loop {
        let question = match term.read_line() {
            Ok(s) => s.trim().to_string(),
            Err(_) => break,
        };

        if question.is_empty() {
            continue;
        }
        if matches!(question.to_lowercase().as_str(), "exit" | "quit" | ":q") {
            break;
        }

        let chunks = rag.retrieve(&question, 5);
        let messages = build_chat_prompt(&question, &format_context(&chunks), &cfg.language, &history);
        let answer = llm.complete(&messages, 0.3, 2048).await?;

        history.push(ChatMessage {
            role: "user".to_string(),
            content: question,
        });
        history.push(ChatMessage {
            role: "assistant".to_string(),
            content: answer.clone(),
        });

        println!("\n{}\n", answer);
    }

    Ok(())
}

fn cmd_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Set { key, value } => {
            let mut cfg = Config::load();
            let value = if key == "model" {
                resolve_model(&value)
            } else {
                value
            };

            match key.as_str() {
                "model" => cfg.model = value.clone(),
                "api_key" => cfg.api_key = value.clone(),
                "api_base" => cfg.api_base = value.clone(),
                "language" => cfg.language = value.clone(),
                "output_dir" => cfg.output_dir = value.clone(),
                _ => bail!("Unknown config key: {}. Valid keys: model, api_key, api_base, language, output_dir", key),
            }

            cfg.save()?;
            println!("{} {} = {}", style("Set").green(), key, value);
        }
        ConfigAction::Get { key } => {
            let cfg = Config::load();
            let val = match key.as_str() {
                "model" => cfg.model.clone(),
                "api_key" => {
                    if cfg.api_key.is_empty() {
                        String::new()
                    } else {
                        let k = &cfg.api_key;
                        if k.len() > 12 {
                            format!("{}...{}", &k[..8], &k[k.len() - 4..])
                        } else {
                            "***".to_string()
                        }
                    }
                }
                "api_base" => cfg.api_base.clone(),
                "language" => cfg.language.clone(),
                "output_dir" => cfg.output_dir.clone(),
                _ => bail!("Unknown config key: {}", key),
            };
            println!("{} = {}", key, val);
        }
        ConfigAction::List => {
            let cfg = Config::load();
            println!("{}", style("Configuration").bold());
            println!("{:<15} {:<40} {}", "Key", "Value", "Source");
            println!("{}", "-".repeat(70));

            let api_key_display = if cfg.api_key.is_empty() {
                String::new()
            } else {
                let k = &cfg.api_key;
                if k.len() > 12 {
                    format!("{}...{}", &k[..8], &k[k.len() - 4..])
                } else {
                    "***".to_string()
                }
            };

            println!("{:<15} {:<40} {}", style("model").cyan(), cfg.model, "default");
            println!("{:<15} {:<40} {}", style("api_key").cyan(), api_key_display, "default");
            println!("{:<15} {:<40} {}", style("api_base").cyan(), cfg.api_base, "default");
            println!("{:<15} {:<40} {}", style("language").cyan(), cfg.language, "default");
            println!("{:<15} {:<40} {}", style("output_dir").cyan(), cfg.output_dir, "default");
        }
    }

    Ok(())
}
