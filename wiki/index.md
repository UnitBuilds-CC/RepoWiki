# index

> Parses source code into structured metadata to provide precise, cost-effective context for LLM-driven documentation.

The module replaces full-codebase dumps with targeted structural indexes. It processes each file through language-specific extractors that capture definitions, parameters, fields, docstrings, and call sites. These extractions are normalized into a unified ProjectIndex, resolved for cross-file dependencies, and grouped into logical modules. A call-flow tracer links symbols to execution paths. Downstream components query this index to inject only relevant code snippets into LLM prompts, minimizing token usage while preserving architectural context. Content hashing prevents redundant parsing of unchanged files.

## Files

### `crates/index/Cargo.toml`

Defines crate entry point and declares dependencies for regex parsing, core model types, and caching infrastructure.

### `crates/index/src/extract.rs`

Implements line-based, language-specific parsers for Python, Rust, TypeScript, Go, and Java. Extracts symbols, imports, parameters, struct/class fields, docstrings, and call sites. Computes cyclomatic complexity and file metrics to quantify code density.

- `extract_symbols` (function) - Public dispatcher that routes raw source content to the correct language extractor based on detected syntax.
- `compute_metrics` (function) - Calculates file-level statistics including cyclomatic complexity and line counts for prioritization.
- `extract_python` (function) - Parses Python files to collect classes, functions, imports, docstrings, and method calls using indentation-aware scanning.
- `extract_rust` (function) - Extracts Rust modules, structs, impl blocks, functions, macros, and trait implementations via pattern matching.
- `extract_typescript` (function) - Captures TS interfaces, classes, methods, enums, and ES module imports using brace/indent tracking.
- `extract_go` (function) - Scans Go files for packages, structs, methods, interfaces, and import declarations.
- `extract_java` (function) - Processes Java classes, methods, fields, annotations, and package imports with visibility detection.

### `crates/index/src/flow.rs`

Constructs directed call graphs by tracing function and method invocations across extracted symbols. Enables behavioral queries without loading full source files.

- `trace_flow` (function) - Analyzes extracted call sites to build a graph of execution dependencies between symbols.
- `build_outgoing` (function) - Generates CallEdge records mapping caller functions to their invoked callees.

### `crates/index/src/lib.rs`

Main orchestrator that coordinates caching, file iteration, symbol extraction, import resolution, and call graph assembly. Groups indexed files into ModuleIndex objects and generates formatted context strings optimized for LLM injection.

- `build_index` (function) - Entry point that iterates over files, applies caching, and returns a complete ProjectIndex.
- `index_one_file` (function) - Handles single-file processing: checks cache, runs extractor, resolves imports, and computes metrics.
- `build_symbol_index` (function) - Creates a reverse lookup map from symbol names to file paths for fast resolution.
- `build_call_graph` (function) - Aggregates flow data from all files into a unified call relationship structure.
- `group_into_module_indices` (function) - Clusters related IndexedFiles into logical ModuleIndex units based on directory and naming conventions.
- `format_module_context` (function) - Serializes a ModuleIndex into a compact, LLM-friendly string containing summaries, symbols, and edges.

### `crates/index/src/resolve.rs`

Maps import statements to actual module or file paths. Handles language-specific resolution rules to prevent broken references and ensure accurate module grouping.

- `resolve_imports` (function) - Batch resolver that normalizes all import paths across the indexed project.
- `resolve_one` (function) - Attempts to match a single import statement against candidate paths using language-specific heuristics.
- `extract_module_path` (function) - Strips syntax prefixes and qualifiers to isolate the canonical module identifier.
- `build_candidates` (function) - Generates filesystem and namespace permutations for an import path to maximize resolution success.

## Key Concepts

- **Structured Metadata Extraction**: Replaces raw text with typed definitions to eliminate irrelevant context, directly cutting LLM token costs.
- **Line-Based Syntax Parsing**: Uses regex and indentation tracking instead of heavy AST compilers, enabling fast, low-memory processing across multiple languages.
- **Call Flow Tracing**: Maps invocation relationships to answer behavioral questions ('how does X work?') without requiring full file loads.
- **Import Normalization**: Aligns disparate import syntaxes into a unified module graph, ensuring accurate grouping and reference resolution.
- **Content-Hashed Caching**: Skips unchanged files using cryptographic hashes, maintaining incremental indexing performance for large repositories.

## Internal Relationships

- `lib.rs` → `extract.rs`: Delegates per-file parsing to language-specific extractors via index_one_file.
- `lib.rs` → `resolve.rs`: Invokes resolve_imports during indexing to link symbols across files and validate module boundaries.
- `lib.rs` → `flow.rs`: Feeds extracted call sites into trace_flow to construct the global call graph.
- `extract.rs` → `flow.rs`: Provides raw call site data that flow.rs converts into directed CallEdge structures.
- `All files` → `repowiki_core::models`: Share immutable type definitions for IndexedSymbol, IndexedImport, FileMetrics, CallEdge, and ProjectIndex.
