# RepoWiki

# RepoWiki

> Rust tool generating structured wiki docs for codebases using indexed LLM analysis.

RepoWiki is a Rust rewrite of an original Python project designed to automatically generate comprehensive wiki documentation for software repositories. It operates through a terminal or built-in web server, producing Markdown, HTML, or JSON output without external runtimes or Docker containers.

The architecture replaces raw source ingestion with a deterministic pre-indexing step that extracts symbols, imports, and dependency graphs. This structured index feeds language models, reducing token consumption by roughly 85 percent per module while keeping descriptions grounded in actual code.

It supports six programming languages, respects .gitignore rules, caches LLM responses in a local SQLite database, and includes auto-generated Mermaid diagrams, PageRank-based reading paths, and cross-linked symbol references.

## Tech Stack

- **Rust** 1.75+ (language)
- **TypeScript** 6.0.2 (language)
- **Axum** 0.8 (framework)
- **Tokio** 1 (runtime)
- **SQLite** bundled (database)
- **Vite** 8.0.8 (build_tool)
- **React** 19.2.5 (library)

## Key Features

- Indexed analysis cuts LLM token costs by approximately 85 percent
- Single static binary with zero runtime dependencies
- Supports Python, JS/TS, Go, Rust, Java, and C/C++
- Auto-generated Mermaid architecture diagrams and PageRank reading paths
- Cross-linked symbol pages and relative navigation in exports
- Incremental re-runs with local SQLite caching
- CLI and web server interfaces with WebSocket chat support

## Getting Started

1. Clone the repository and navigate to the root directory.
2. Install the Rust toolchain if not already present.
3. Run cargo build --release to compile the binary.
4. Set an LLM provider API key via environment variable or repowiki config set api_key.
5. Execute repowiki scan <path_or_url> to generate documentation.

## Contents

- [Overview](index)
- [Architecture](architecture)
- **Modules**
  - [frontend](frontend)
  - [server](server)
  - [export](export)
  - [root](root)
  - [index](index)
  - [core](core)
  - [ingest](ingest)
  - [llm](llm)
  - [scanner](scanner)
  - [.github](.github)
  - [analyzer](analyzer)
  - [cache](cache)
  - [cli](cli)
  - [graph](graph)
  - [rag](rag)
- [Reading Guide](reading-guide)
- [Symbol Index](symbols)
