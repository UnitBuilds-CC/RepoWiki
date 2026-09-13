# index

> Parses, resolves, and structures source code into a navigable project index for downstream LLM documentation generation.

The index module transforms raw repository files into a unified, queryable representation. It handles multi-language symbol extraction without relying on heavy AST libraries, resolves cross-file imports, constructs a call graph for dependency tracing, and groups files into logical modules. This structured output provides the foundational context required for accurate, code-aware LLM documentation generation.

## Files

### `crates/index/Cargo.toml`

Declares crate metadata and runtime dependencies including regex parsing utilities and core model definitions.

### `crates/index/src/extract.rs`

Implements language-specific parsers for Python, Rust, TypeScript, Go, and Java to extract symbols, imports, parameters, docstrings, and structural fields.

- `extract_symbols` (function) - Dispatches to language-specific extractors based on file extension.
- `compute_metrics` (function) - Calculates complexity and structural metrics for a given code snippet.
- `extract_python` (function) - Parses Python source to return indexed symbols and imports.
- `extract_rust` (function) - Parses Rust source to return indexed symbols and imports.
- `extract_typescript` (function) - Parses TypeScript source to return indexed symbols and imports.
- `extract_go` (function) - Parses Go source to return indexed symbols and imports.
- `extract_java` (function) - Parses Java source to return indexed symbols and imports.

### `crates/index/src/flow.rs`

Traces function/method calls between symbols to build a directed call graph representing execution dependencies.

- `trace_flow` (function) - Analyzes extracted symbols to generate CallEdge objects mapping caller-callee relationships.
- `build_outgoing` (function) - Constructs outgoing call edges for a specific symbol based on detected invocation patterns.

### `crates/index/src/lib.rs`

Orchestrates the entire indexing pipeline, coordinating file reading, caching, extraction, import resolution, call graph generation, and module grouping.

- `build_index` (function) - Public entry point that processes a file list and returns a complete ProjectIndex.
- `index_one_file` (function) - Handles single-file processing including cache validation and language dispatch.
- `build_symbol_index` (function) - Creates a fast lookup map from symbol names to their defining file paths.
- `group_into_module_indices` (function) - Clusters related files into logical ModuleIndex units based on naming and path conventions.
- `format_module_context` (function) - Generates a concise textual summary of a module's contents for prompt injection.

### `crates/index/src/resolve.rs`

Resolves abstract import statements to concrete file paths using language-specific heuristics and path normalization.

- `resolve_imports` (function) - Maps all extracted imports across the project to their actual source locations.
- `resolve_python_module` (function) - Applies Python sys.path and relative import logic to locate target modules.
- `build_candidates` (function) - Generates potential file paths for an import based on source location and language rules.

## Key Concepts

- **Regex-Based Language Parsing**: Avoids heavy AST compilation by using targeted regex and indentation analysis for rapid, broad-spectrum symbol extraction across multiple languages.
- **Content-Addressed Caching**: Uses content hashes to skip unchanged files during indexing, significantly reducing rebuild times for large repositories.
- **Call Graph Construction**: Transforms flat symbol lists into relational edges, enabling traversal of execution paths and dependency chains for contextual documentation.
- **Logical Module Grouping**: Aggregates individual files into cohesive modules based on naming/path patterns, providing higher-level context for LLM prompts.

## Internal Relationships

- `crates/index/src/lib.rs` → `crates/index/src/extract.rs`: lib.rs delegates raw source parsing to extract.rs to populate IndexedFile structures.
- `crates/index/src/lib.rs` → `crates/index/src/resolve.rs`: lib.rs passes unresolved imports from extract.rs to resolve.rs to establish cross-file links.
- `crates/index/src/lib.rs` → `crates/index/src/flow.rs`: lib.rs uses resolved symbols and imports to invoke trace_flow, constructing the dependency graph.
- `crates/index/src/extract.rs` → `crates/index/src/resolve.rs`: extract.rs produces IndexedImport records that serve as input for resolve.rs path resolution.
- `crates/index/src/lib.rs` → `crates/index/Cargo.toml`: lib.rs depends on dependencies declared in Cargo.toml, specifically regex and repowiki_core models.
