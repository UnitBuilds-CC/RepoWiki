# cli

> Provides the unified entry point for both CLI and web interfaces to orchestrate codebase scanning, LLM-driven analysis, and wiki generation.

The cli module acts as the application facade, translating user input into actionable workflows across the repowiki ecosystem. It leverages clap for declarative argument parsing and tokio for asynchronous task execution. The module routes commands to specialized subroutines that coordinate scanning, dependency graphing, RAG indexing, LLM prompting, and static site generation. It also initializes a shared AppState for the web server, ensuring consistent configuration and caching behavior across both interfaces. By centralizing orchestration here, the core logic remains decoupled from UI concerns while maintaining a single source of truth for workflow execution.

## Files

### `crates/cli/Cargo.toml`

Declares runtime dependencies for the CLI binary crate, linking it to all repowiki subsystems (core, scanner, analyzer, llm, wiki, server, cache, index, rag, ingest, graph).

### `crates/cli/src/main.rs`

Implements the application entry point, CLI schema, command routing, and web server initialization. Handles argument parsing, async execution, and delegates to core modules for actual work.

- `Cli` (struct) - Root clap structure defining the top-level CLI schema and routing to subcommands.
- `Commands` (enum) - Defines all available CLI subcommands including map, scan, serve, cache-clear, index, chat, and config.
- `MapFormat` (enum) - Specifies output serialization formats for dependency graphs.
- `ScanFormat` (enum) - Controls how scan reports are serialized for terminal or file output.
- `ConfigAction` (enum) - Enumerates configuration management operations like setting or retrieving API keys.
- `is_url` (function) - Determines whether a provided string is a remote Git URL or a local filesystem path.
- `main` (function) - Application bootstrap: parses arguments, initializes tokio runtime, resolves configuration, and dispatches to the appropriate command handler.
- `cmd_map` (function) - Generates and displays a dependency graph for a target codebase in the specified format.
- `cmd_scan` (function) - Scans a directory tree for code files and outputs a structured scan report.
- `run_analysis` (function) - Orchestrates the full documentation pipeline: loads project context, builds RAG index, invokes the analyzer with LLM prompts, and constructs the final wiki.
- `cmd_serve` (function) - Initializes and starts the HTTP server, binding to a port and serving the web interface with shared application state.
- `cmd_cache_clear` (function) - Removes cached LLM responses and intermediate artifacts to force fresh analysis.
- `cmd_index` (function) - Triggers RAG index construction or rebuild for a given codebase path.
- `cmd_chat` (function) - Enters an interactive REPL loop that queries the LLM using formatted project context and chat history.
- `cmd_config` (function) - Manages persistent configuration actions such as setting API keys or selecting default models.

## Key Concepts

- **Unified Orchestration Layer**: Centralizes workflow coordination so both CLI and web interfaces trigger identical scanning, indexing, and analysis pipelines without duplicating business logic.
- **Async-First Execution**: Leverages tokio to handle blocking I/O (disk scans, network LLM calls, HTTP requests) concurrently, preventing UI freezes and maximizing throughput.
- **State Sharing via AppState**: Wraps configuration, cache, and LLM clients in Arc-wrapped shared state, enabling seamless handoff between CLI sessions and long-running web server instances.
- **RAG-Driven Context Injection**: Bridges raw codebase structure with LLM capabilities by building indexes and formatting retrieved snippets, ensuring analyses are grounded in actual project architecture rather than generic patterns.

## Internal Relationships

- `crates/cli/src/main.rs` → `crates/cli/Cargo.toml`: Main.rs relies on the dependency graph defined in Cargo.toml to link against all repowiki workspace crates.
- `crates/cli/src/main.rs` → `repowiki_scanner::scan_directory`: Delegates file discovery and metadata extraction to the scanner module during cmd_scan and run_analysis.
- `crates/cli/src/main.rs` → `repowiki_analyzer::Analyzer`: Passes scanned code and RAG context to the analyzer for LLM-driven documentation generation.
- `crates/cli/src/main.rs` → `repowiki_llm::client::LLMClient`: Instantiates and manages the LLM client connection, handling prompt construction and response streaming for chat and analysis.
- `crates/cli/src/main.rs` → `repowiki_server::{AppState, create_app}`: Constructs shared application state and mounts route handlers to expose the web interface over TCP.
- `crates/cli/src/main.rs` → `repowiki_rag::{format_context, load_or_build_index}`: Uses RAG utilities to chunk code, build vector/text indices, and format retrieved snippets for LLM prompts.
