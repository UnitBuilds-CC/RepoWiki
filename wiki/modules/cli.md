# cli

> Provides the primary entry point for interacting with the repowiki system via terminal commands and a web interface, routing user input to specialized analysis, indexing, and LLM pipelines.

The cli module acts as the application facade and command router. It parses terminal arguments using clap, initializes shared state (configuration, cache, LLM client), and dispatches operations to domain-specific crates. It supports direct codebase scanning, dependency mapping, interactive LLM chat, configuration management, and a persistent web server for browser-based interaction. A core design goal is cost optimization: the module triggers structured indexing before LLM calls so the model only processes relevant code snippets instead of full repositories, significantly reducing token consumption.

## Files

### `crates/cli/Cargo.toml`

Declares dependencies for the CLI binary crate, linking to all repowiki submodules and runtime libraries required for async execution, HTTP serving, and argument parsing.

### `crates/cli/src/main.rs`

Implements the CLI argument parser, command handlers, and web server initialization. Orchestrates the end-to-end wiki generation workflow by delegating to scanner, analyzer, indexer, and LLM crates.

- `Cli` (struct) - Root clap struct defining the top-level CLI interface and routing to subcommands.
- `Commands` (enum) - Defines available subcommands: map, scan, serve, cache-clear, index, chat, and config.
- `cmd_map` (function) - Generates a visual dependency graph of the target repository and outputs it in formats like Mermaid or JSON.
- `cmd_scan` (function) - Traverses the filesystem to produce a structured ScanReport containing file paths, languages, and metadata.
- `run_analysis` (function) - Orchestrates the core pipeline: scans the repo, builds the cost-optimized index, feeds context to the Analyzer and WikiBuilder, and generates documentation.
- `cmd_serve` (function) - Initializes an Actix web server with shared AppState, exposing HTTP endpoints for browser-based wiki generation and chat.
- `cmd_cache_clear` (function) - Deletes cached analysis results and LLM responses to force a complete regeneration.
- `cmd_index` (function) - Builds or loads the structured code index, which maps symbols and relationships to minimize tokens sent to the LLM.
- `cmd_chat` (function) - Runs an interactive REPL-style session that queries the LLM using indexed context and formatted prompts.
- `cmd_config` (function) - Manages LLM provider settings and model selection, resolving configurations at runtime.

## Key Concepts

- **Cost-Optimized Structured Indexing**: Before invoking expensive LLM calls, the CLI builds a lightweight index of code structure and dependencies. This allows the model to retrieve only relevant snippets via RAG, cutting token usage by orders of magnitude compared to raw context injection.
- **Shared Async State Management**: Uses std::sync::Arc to safely distribute configuration, cache, and LLM client instances across synchronous CLI commands and asynchronous HTTP handlers without race conditions.
- **Unified Pipeline Abstraction**: Both terminal commands and the web server route through the same run_analysis flow. This ensures feature parity between CLI and web interfaces while centralizing business logic in downstream crates.

## Internal Relationships

- `crates/cli/src/main.rs` → `crates/cli/Cargo.toml`: Cargo.toml declares the external crates and workspace dependencies that main.rs compiles against and links at runtime.
- `crates/cli/src/main.rs` → `repowiki_scanner`: cmd_scan delegates filesystem traversal and metadata extraction to the scanner crate, returning a ScanReport.
- `crates/cli/src/main.rs` → `repowiki_index`: run_analysis and cmd_index call build_index/load_or_build_index to create the structured lookup table that reduces LLM token consumption.
- `crates/cli/src/main.rs` → `repowiki_analyzer`: run_analysis passes indexed context and scanned files to the Analyzer crate to extract architectural insights before wiki generation.
- `crates/cli/src/main.rs` → `repowiki_llm`: All AI-dependent commands (run_analysis, cmd_chat, cmd_serve) initialize an LLMClient and use build_chat_prompt/format_context to optimize prompt payloads.
- `crates/cli/src/main.rs` → `repowiki_server`: cmd_serve constructs an AppState sharing cache, config, and LLM client references across HTTP workers, bridging the CLI pipeline to the web interface.
