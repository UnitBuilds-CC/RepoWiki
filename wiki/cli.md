# cli

> Provides the unified command-line interface for triggering codebase analysis, documentation generation, and interactive LLM querying within the Repowiki ecosystem.

The cli module acts as the application entry point and user-facing orchestrator. It parses CLI arguments via clap, routes commands to specialized handlers, and coordinates cross-crate workflows. It manages the complete lifecycle of wiki generation: ingesting repositories, scanning directories, building dependency graphs, caching intermediate results, invoking LLM clients for context-aware analysis, and optionally serving the output via an HTTP server. The high cyclomatic complexity reflects extensive branching for format selection, error recovery, and multi-mode operation.

## Files

### `crates/cli/Cargo.toml`

Declares binary crate metadata and runtime dependencies required for the CLI executable.

### `crates/cli/src/main.rs`

Implements argument parsing, command dispatch, and orchestration logic. Bridges user input to core analysis, LLM, caching, and web-serving subsystems.

- `Cli` (struct) - Root structure defining the top-level CLI schema parsed by clap.
- `Commands` (enum) - Maps CLI subcommands to their corresponding handler functions.
- `cmd_map` (function) - Generates and outputs a dependency visualization or summary for a specified project path.
- `cmd_scan` (function) - Scans a directory tree, collects file metadata, and produces a structured scan report.
- `run_analysis` (function) - Orchestrates the core workflow: loads project context, formats prompts, queries the LLM, and constructs wiki pages.
- `cmd_serve` (function) - Initializes and runs an asynchronous HTTP server to expose the generated wiki and API endpoints.
- `cmd_cache_clear` (function) - Purges the local caching layer storing intermediate analysis and LLM responses.
- `cmd_index` (function) - Builds or refreshes the semantic index used for RAG-based code retrieval.
- `cmd_chat` (function) - Starts an interactive REPL session with the LLM, injecting retrieved code context into prompts.
- `cmd_config` (function) - Manages persistent configuration, including model resolution and credential storage.

## Key Concepts

- **Command Routing**: Centralized dispatch pattern separates CLI parsing from business logic, allowing each subcommand to focus on a single responsibility while maintaining a clean entry point.
- **Async Orchestration**: Uses tokio to run long-running operations like LLM inference, index building, and web serving concurrently without blocking the terminal UI.
- **Stateful Caching**: Persists intermediate scan results and LLM outputs locally to reduce API costs, enable incremental updates, and prevent redundant computation.
- **Context-Aware Analysis**: Integrates RAG pipelines to inject relevant code snippets into LLM prompts, ensuring generated wiki documentation is grounded in the actual codebase topology.

## Internal Relationships

- `crates/cli/src/main.rs` → `repowiki_core`: Loads application configuration and resolves target LLM models based on user settings.
- `crates/cli/src/main.rs` → `repowiki_ingest`: Delegates Git URL validation and repository ingestion to handle remote sources.
- `crates/cli/src/main.rs` → `repowiki_scanner`: Offloads directory traversal and file metadata collection to generate scan reports.
- `crates/cli/src/main.rs` → `repowiki_analyzer`: Passes scanned data and LLM prompts to the analyzer for structured wiki generation.
- `crates/cli/src/main.rs` → `repowiki_llm`: Instantiates and communicates with the LLM client to execute chat completions and prompt rendering.
- `crates/cli/src/main.rs` → `repowiki_cache`: Reads and writes cached scan results and LLM responses to minimize redundant API calls and rebuilds.
- `crates/cli/src/main.rs` → `repowiki_server`: Spawns the async Tokio listener and mounts AppState to serve the final wiki over HTTP.
- `crates/cli/src/main.rs` → `repowiki_rag`: Formats retrieved code context and loads the vector index to ground LLM queries in actual project structure.
