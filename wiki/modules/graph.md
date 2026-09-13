# graph

> Analyzes codebase structure to prioritize documentation targets and optimize LLM indexing.

Constructs a directed dependency graph from a codebase to identify architectural relationships, compute file importance via PageRank, and detect structural patterns like entry points, isolated modules, and circular dependencies. The module parses language-specific import statements, normalizes paths, and provides analytical queries and Mermaid visualization output. It serves as the structural foundation for the documentation generator, enabling it to focus LLM context windows on high-value files first while avoiding redundant or peripheral code.

## Files

### `crates/graph/Cargo.toml`

Declares runtime dependencies required for graph algorithms, regex parsing, and core model integration.

### `crates/graph/src/lib.rs`

Implements the dependency graph builder, analyzers, and exporters.

- `DependencyGraph` (struct) - Wraps a petgraph DiGraph to represent files as nodes and imports as directed edges.
- `build_from_project` (method) - Parses ProjectContext to extract imports using language-specific regex patterns and constructs the dependency graph.
- `rank_files` (method) - Computes PageRank scores for all nodes to quantify file importance and connectivity.
- `get_core_files` (method) - Returns the top N highest-ranked files based on PageRank scores.
- `get_module_dependencies` (method) - Aggregates dependencies by logical module rather than individual files.
- `to_mermaid` (method) - Serializes the graph into Mermaid.js flowchart syntax for visual inspection.
- `get_entry_points` (method) - Identifies nodes with zero incoming edges, representing public APIs or binaries.
- `find_isolated_files` (method) - Locates nodes with no incoming or outgoing edges, indicating orphaned or standalone code.
- `find_circular_dependencies` (method) - Uses Tarjan's strongly connected components algorithm to detect and return cycles up to a specified limit.
- `nodes` (method) - Extracts all node identifiers from the graph.
- `edges` (method) - Extracts all directed edge pairs from the graph.
- `pagerank_power_iteration` (function) - Implements the iterative PageRank algorithm to converge on stable importance scores.
- `import_patterns` (function) - Returns language-specific regex patterns for extracting import statements.
- `resolve_import` (function) - Matches an import statement against known patterns and resolves it to a target file path.
- `resolve_python_module` (function) - Specialized resolver for Python import paths, handling relative imports and module-to-file mapping.
- `normalize_path` (function) - Sanitizes and standardizes file paths to ensure consistent node identification.
- `get_module` (function) - Derives a logical module name from a file path.
- `mermaid_id` (function) - Escapes and formats file names into valid Mermaid node identifiers.

## Key Concepts

- **PageRank Prioritization**: Scores files by connectivity to determine which modules are most critical for documentation, reducing unnecessary LLM calls on peripheral code.
- **Directed Dependency Modeling**: Represents imports as directed edges to accurately capture code flow and architectural boundaries.
- **Language-Agnostic Import Resolution**: Uses extensible regex patterns and specialized resolvers to map import strings to physical files regardless of language.
- **Structural Analysis Queries**: Provides targeted graph operations to surface architectural health and inform documentation strategy.

## Internal Relationships

- `crates/graph/src/lib.rs` → `repowiki_core::models::ProjectContext`: Consumes ProjectContext to access file lists and metadata during graph construction.
- `crates/graph/src/lib.rs` → `petgraph`: Relies on petgraph for DiGraph storage, EdgeRef iteration, and Tarjan SCC cycle detection.
- `crates/graph/src/lib.rs` → `regex`: Uses compiled regex patterns to parse language-specific import syntax across different codebases.
- `crates/graph/src/lib.rs` → `upstream documentation pipeline`: Exports ranked files, core modules, and Mermaid diagrams to guide LLM context allocation and wiki generation.
