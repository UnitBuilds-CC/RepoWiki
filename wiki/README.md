# RepoWiki

# RepoWiki

> Generates structured wiki documentation for any codebase via CLI or web interface using LLM analysis.

RepoWiki is a Rust-based tool that automatically creates detailed documentation for software projects. It scans local directories or GitHub repositories, analyzes the code structure, and produces organized wiki pages covering project overviews, module details, architecture diagrams, and symbol indexes.

Unlike the original Python version, this rewrite uses an indexed-analysis architecture. It builds deterministic summaries of symbols, imports, and call graphs before sending data to large language models. This approach reduces token usage by roughly 85 percent per module while maintaining accurate, code-grounded descriptions.

The project provides multiple output formats including Markdown, HTML, and GitHub Pages-ready sites. It includes a built-in web server for interactive browsing, a chat interface for asking questions about the codebase, and supports incremental updates to avoid regenerating unchanged pages.

## Tech Stack

- **Rust** 1.75+ (language)
- **TypeScript** 6.0.2 (language)
- **Axum** 0.8 (framework)
- **React** 19.2.5 (library)
- **Vite** 8.0.8 (build_tool)
- **SQLite** bundled (database)

## Key Features

- Indexed analysis feeds compact structured summaries to LLMs instead of raw source code.
- Auto-generated architecture diagrams using Mermaid and PageRank-based reading paths.
- Cross-linked wiki pages with automatic symbol and file path linking.
- Global symbol index grouped by kind and module.
- Incremental re-runs that track page inputs to skip unchanged sections.
- Support for six programming languages: Python, JS/TS, Go, Rust, Java, C/C++.
- Multiple export formats: Markdown, JSON, HTML, and GitHub Pages.

## Getting Started

1. Clone the repository and navigate into the directory.
2. Build the release binary using cargo build --release.
3. Set your LLM API key via environment variable or run repowiki config set api_key <your-key>.
4. Run repowiki scan ./path-to-project to generate documentation or repowiki serve ./path-to-project to start the web interface.

## Contents

- [Overview](index)
- [Architecture](architecture)
- **Modules**
  - [frontend](frontend)
  - [server](server)
  - [export](export)
  - [root](root)
  - [index](index)
  - [ingest](ingest)
  - [llm](llm)
  - [scanner](scanner)
  - [core](core)
  - [.github](.github)
  - [analyzer](analyzer)
  - [cache](cache)
  - [cli](cli)
  - [graph](graph)
  - [rag](rag)
- [Reading Guide](reading-guide)
- [Symbol Index](symbols)
