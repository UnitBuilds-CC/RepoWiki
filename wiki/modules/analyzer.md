# analyzer

> Orchestrates LLM-driven codebase analysis to generate structured wiki documentation while minimizing token costs through caching and indexed context.

The analyzer module coordinates the end-to-end generation of technical documentation by decomposing a codebase into manageable units. It leverages pre-built indexes and dependency graphs to construct highly targeted LLM prompts, drastically reducing context window usage. Concurrency is managed via a semaphore to prevent resource exhaustion, while a content-aware cache stores and retrieves LLM responses to eliminate redundant API calls. The module exposes a single analyze entry point that sequences these steps and emits progress updates via a callback.

## Files

### `crates/analyzer/Cargo.toml`

Defines Rust crate metadata and declares external dependencies for async execution, LLM clients, caching, indexing, and graph traversal.

### `crates/analyzer/src/lib.rs`

Implements the core analysis orchestration logic, prompt construction, concurrency management, and caching integration.

- `ProgressFn` (type_alias) - Callback signature for reporting incremental analysis progress to CLI or web interfaces.
- `Analyzer` (struct) - Stateful orchestrator holding references to the LLM client, cache, concurrency semaphore, language, and output prefix.
- `new` (method) - Constructor initializing the analyzer with required dependencies and configurable concurrency limits.
- `analyze` (method) - Top-level workflow coordinator that triggers overview, module, architecture, and reading guide generation sequentially.
- `generate_overview` (method) - Produces a high-level project summary using aggregated index data and overview-specific prompts.
- `analyze_modules` (method) - Iterates through the project index, delegating individual module analysis to analyze_one_module while respecting concurrency limits.
- `generate_architecture` (method) - Constructs system-level design documentation by traversing the dependency graph and injecting it into architecture prompts.
- `generate_reading_guide` (method) - Outputs a curated sequence of modules and files to help developers navigate the codebase logically based on dependencies.
- `build_key_files_context` (function) - Formats critical entry-point files into a concise string for prompt injection, reducing raw code bloat.
- `build_module_summary` (function) - Extracts and formats structural metadata from the project index to provide lightweight context for LLM queries.
- `analyze_one_module` (function) - Executes the actual LLM call for a single module, applying caching, prompt templating, and JSON extraction.

## Key Concepts

- **Cost-Optimized Prompting**: Uses structured indexes and dependency graphs instead of raw source code to keep LLM context windows small, predictable, and cheap.
- **Content-Aware Caching**: Hashes prompt inputs to cache responses, eliminating redundant LLM calls across runs, retries, or concurrent requests.
- **Bounded Concurrency**: Employs a tokio Semaphore to limit parallel LLM requests, preventing rate limits, memory spikes, and server overload.
- **Modular Decomposition**: Splits documentation generation into discrete phases (overview, modules, architecture, guide) for focused prompting, easier debugging, and incremental progress reporting.

## Internal Relationships

- `crates/analyzer/Cargo.toml` → `crates/analyzer/src/lib.rs`: Declares dependencies and feature flags required to compile the implementation.
- `crates/analyzer/src/lib.rs` → `repowiki_index`: Consumes parsed codebase indexes to build lightweight, cost-effective prompt contexts instead of dumping raw source files.
- `crates/analyzer/src/lib.rs` → `repowiki_graph`: Uses dependency relationships to generate architecture and reading guide documentation.
- `crates/analyzer/src/lib.rs` → `repowiki_cache`: Stores and retrieves LLM responses keyed by content hashes to avoid duplicate API calls across runs.
- `crates/analyzer/src/lib.rs` → `repowiki_llm`: Delegates prompt formatting and actual inference requests to specialized client and prompt modules.
