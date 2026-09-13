# RepoWiki

# RepoWiki

> Generates wiki documentation for any codebase via CLI or web interface using structured indexing to cut LLM costs.

RepoWiki is a Rust-based tool that automatically generates comprehensive wiki documentation for software repositories. It scans local directories or GitHub URLs, analyzes the code structure, and produces structured markdown, HTML, or JSON output. The application compiles to a single static binary and requires no external runtime dependencies.

Unlike tools that feed raw source code to large language models, RepoWiki uses a deterministic pre-indexing step. It extracts symbols, imports, and dependency graphs to create compact summaries. This indexed analysis reduces token consumption by approximately 85 percent per module while keeping descriptions grounded in actual code.

The project features a full-stack architecture with a Rust backend handling scanning, caching, graph analysis, and LLM integration. A React frontend provides an interactive web interface for browsing generated wikis and asking questions about the codebase. It supports six programming languages, respects gitignore patterns, and caches results for fast incremental updates.

## Tech Stack

- **Rust** 2021 (language)
- **Axum** 0.8 (framework)
- **Tokio** 1 (runtime)
- **Rusqlite** 0.32 (database)
- **Petgraph** 0.7 (library)
- **React** 19.2 (framework)
- **Vite** 8.0 (build_tool)
- **Tailwind CSS** 4.2 (css_framework)
- **Mermaid** 11.14 (visualization)
- **Zustand** 5.0 (state_management)

## Key Features

- Indexed analysis reduces LLM token usage by approximately 85 percent per module
- Supports six programming languages including Python, JavaScript, TypeScript, Go, Rust, Java, and C/C++
- Generates cross-linked markdown, self-contained HTML, and JSON exports
- Auto-detects project architecture and renders dependency diagrams using Mermaid
- Caches analysis results for fast incremental re-runs without regenerating unchanged pages
- Provides a built-in web server with interactive wiki browsing and codebase chat

## Getting Started

1. Clone the repository and compile the release binary with cargo build --release
2. Set your LLM provider API key using export DEEPSEEK_API_KEY=<key> or repowiki config set api_key <key>
3. Run repowiki scan ./path-to-project to generate documentation in the default markdown format
4. Optionally run repowiki serve ./path-to-project to launch the interactive web interface on port 8000

## Contents

- [Overview](index.md)
- [Architecture](architecture.md)
- **Modules**
  - [frontend](modules/frontend.md)
  - [server](modules/server.md)
  - [root](modules/root.md)
  - [export](modules/export.md)
  - [index](modules/index.md)
  - [core](modules/core.md)
  - [ingest](modules/ingest.md)
  - [llm](modules/llm.md)
  - [scanner](modules/scanner.md)
  - [.github](modules/.github.md)
  - [analyzer](modules/analyzer.md)
  - [cache](modules/cache.md)
  - [cli](modules/cli.md)
  - [graph](modules/graph.md)
  - [rag](modules/rag.md)
- [Reading Guide](reading-guide.md)
- [Symbol Index](symbols.md)
