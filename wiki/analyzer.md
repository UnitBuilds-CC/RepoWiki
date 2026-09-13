# analyzer

> Orchestrates LLM-driven analysis of a codebase to generate structured wiki documentation across multiple phases.

The analyzer module coordinates the entire documentation generation pipeline by breaking it into discrete stages: project overview, per-module analysis, architecture mapping, and developer reading guides. It manages concurrency via semaphores, caches LLM responses using content hashes to prevent redundant calls, and dynamically assembles contextual prompts from the codebase index and dependency graph. The module abstracts away LLM interaction details, ensuring consistent JSON output and efficient resource utilization during large-scale codebase analysis.

## Files

### `crates/analyzer/Cargo.toml`

Declares runtime dependencies for the analyzer crate, including LLM clients, caching mechanisms, indexing utilities, and async concurrency primitives.

### `crates/analyzer/src/lib.rs`

Contains the complete implementation of the documentation generation orchestrator, including state management, concurrent execution control, prompt assembly helpers, and LLM response processing.

- `ProgressFn` (type) - Callback type signature for reporting analysis progress to callers.
- `Analyzer` (struct) - Main orchestrator holding LLM client, cache, language config, concurrency semaphore, and cache key registry.
- `new` (method) - Constructor that initializes the analyzer with injected dependencies and sets concurrency limits.
- `analyze` (method) - Top-level entry point that sequentially executes overview, module, architecture, and reading guide generation phases.
- `generate_overview` (method) - Constructs and sends a prompt to generate a high-level project summary using the LLM.
- `analyze_modules` (method) - Iterates through the project index, builds contextual summaries, and spawns concurrent tasks to analyze each module.
- `generate_architecture` (method) - Analyzes the dependency graph to produce structural documentation about component relationships.
- `generate_reading_guide` (method) - Generates a prioritized sequence of files/modules recommended for new developers to understand the codebase.
- `build_key_files_context` (function) - Extracts and formats critical project files into a concise string for LLM context injection.
- `build_module_summary` (function) - Condenses the project index into a structured summary to reduce token usage during module analysis.
- `analyze_one_module` (function) - Handles single-module analysis: checks cache, constructs prompt, calls LLM, parses JSON response, and stores result.

## Key Concepts

- **Phased Documentation Pipeline**: Splits analysis into sequential stages (overview, modules, architecture, reading guide) to maintain focus, improve LLM accuracy, and allow incremental output.
- **Content-Addressed Caching**: Uses cryptographic hashing of source content to deduplicate work, ensuring identical code never triggers redundant LLM calls.
- **Bounded Concurrency**: Employs a semaphore to limit parallel analysis tasks, balancing throughput with API rate limits and system stability.
- **Dynamic Context Assembly**: Programmatically extracts and formats only relevant index/graph data into compact strings, optimizing token usage without losing structural insight.

## Internal Relationships

- `crates/analyzer/src/lib.rs` → `repowiki_llm::client::LLMClient`: Analyzer delegates all prompt construction and response retrieval to the LLM client, enforcing strict JSON parsing.
- `crates/analyzer/src/lib.rs` → `repowiki_cache::Cache`: Analyzer uses content hashing to cache LLM outputs, skipping unchanged code sections to save API costs and latency.
- `crates/analyzer/src/lib.rs` → `repowiki_index::build_index`: Analyzer consumes the pre-built project index to iterate over modules and assemble contextual summaries.
- `crates/analyzer/src/lib.rs` → `repowiki_graph::DependencyGraph`: Analyzer queries the dependency graph to map component relationships for architecture documentation generation.
- `crates/analyzer/src/lib.rs` → `repowiki_llm::prompts`: Analyzer imports prompt builders and JSON extractors to format inputs and parse LLM responses consistently.
- `crates/analyzer/src/lib.rs` → `tokio::sync::Semaphore`: Analyzer uses the semaphore to bound concurrent LLM requests, preventing rate limit violations and memory exhaustion.
