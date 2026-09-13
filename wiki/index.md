# RepoWiki

> Generates structured wiki documentation for any codebase using LLMs and indexed analysis.

RepoWiki is a Rust-based tool that automatically generates comprehensive wiki documentation for software repositories. It replaces raw source code feeding into large language models with a pre-indexed analysis of symbols, imports, and dependency graphs, which drastically reduces token usage while maintaining accuracy.

The system scans local directories or GitHub repositories, extracts structural metadata, and uses configurable LLM providers to produce module-level documentation, architecture diagrams, and cross-linked pages. It supports multiple output formats including Markdown, HTML, and JSON, and includes an incremental caching system to avoid redundant processing.

A companion web interface built with React and Vite provides interactive navigation, real-time chat about the codebase, and Mermaid diagram rendering. The entire backend runs as a single static binary with zero runtime dependencies, making it fast to deploy and run locally.

## Tech Stack

- **Rust** 1.75+ (language)
- **Axum** 0.8 (framework)
- **Tokio** 1 (async runtime)
- **Rusqlite** 0.32 (database)
- **Petgraph** 0.7 (graph library)
- **React** 19.2.5 (frontend framework)
- **TypeScript** 6.0.2 (frontend language)
- **Vite** 8.0.8 (build tool)
- **Tailwind CSS** 4.2.2 (styling)
- **Mermaid** 11.14.0 (diagramming)
- **Shiki** 3.23.0 (syntax highlighting)
- **Zustand** 5.0.12 (state management)
- **Clap** 4 (CLI parser)
- **Reqwest** 0.12 (HTTP client)

## Key Features

- Indexed analysis that cuts LLM token costs by ~85% per module
- Structured wiki generation with auto-detected architecture and Mermaid diagrams
- Cross-linked pages with automatic symbol and file path linking
- Global symbol index grouped by kind and module
- Incremental re-runs that skip unchanged pages using state tracking
- Support for six programming languages including Python, JS/TS, Go, Rust, Java, and C/C++
- Web interface with real-time chat and interactive navigation
- Single static binary deployment with zero runtime dependencies

## Getting Started

1. Clone the repository and navigate to the project directory.
2. Build the release binary using cargo build --release.
3. Set your LLM provider API key via environment variable or repowiki config set api_key <key>.
4. Run repowiki scan ./path-to-project to generate documentation.
