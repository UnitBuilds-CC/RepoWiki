# graph

> Models and analyzes codebase dependency structures to prioritize documentation targets and detect architectural issues.

This module constructs a directed graph representing import/dependency relationships across an entire codebase. It ingests raw project metadata, resolves language-specific imports using regex patterns, and normalizes paths to create a unified DependencyGraph. The module provides analytical methods including PageRank-based file importance scoring, Tarjan's strongly connected components for cycle detection, entry point identification, and isolated file detection. It also exports the graph structure to Mermaid.js syntax for visual embedding in generated wikis. By abstracting cross-language import resolution into a single graph abstraction, it enables downstream LLM documentation pipelines to intelligently prioritize core modules, warn about circular dependencies, and map module boundaries accurately.

## Files

### `crates/graph/Cargo.toml`

Defines crate metadata and declares external dependencies required for graph operations, pattern matching, and core model integration.

### `crates/graph/src/lib.rs`

Core implementation of the DependencyGraph struct and all analysis/export methods. Handles project ingestion, import resolution, graph traversal, ranking, and Mermaid generation.

- `DependencyGraph` (struct) - Holds the underlying petgraph DiGraph and exposes public APIs for graph analysis, ranking, and visualization export.
- `build_from_project` (method) - Parses all files in the ProjectContext, extracts imports using language-specific regex patterns, resolves paths, and populates the directed dependency graph.
- `rank_files` (method) - Computes and returns a sorted list of files with their PageRank scores to quantify module importance and centrality.
- `get_core_files` (method) - Filters the ranked files and returns the top N most important modules based on PageRank thresholds.
- `get_module_dependencies` (method) - Maps each module name to a set of its direct downstream dependencies for quick lookup during documentation generation.
- `to_mermaid` (method) - Transforms internal graph nodes and edges into Mermaid.js flowchart syntax for static wiki visualization.
- `get_entry_points` (method) - Returns files with zero incoming edges, identifying root modules or application entrypoints.
- `find_isolated_files` (method) - Identifies files with no incoming or outgoing dependency edges, flagging them as potentially undocumented or standalone.
- `find_circular_dependencies` (method) - Applies Tarjan's SCC algorithm to detect cyclic import chains, capped at a specified limit to prevent performance degradation.
- `nodes` (method) - Returns a flat list of all file paths represented as graph nodes.
- `edges` (method) - Returns a list of tuples representing directed import relationships between source and target files.
- `pagerank_power_iteration` (function) - Implements the iterative power method algorithm to converge on stable PageRank scores for each node in the graph.
- `import_patterns` (function) - Returns a vector of compiled Regex patterns tailored to specific programming languages for extracting import statements.
- `resolve_import` (function) - Resolves matched import tokens against the project filesystem to produce normalized, absolute file paths.
- `resolve_python_module` (function) - Specialized resolver for Python import syntax, handling relative dots, __init__.py mappings, and package prefixes.
- `normalize_path` (function) - Standardizes file paths by stripping prefixes, resolving symlinks, and ensuring consistent casing for reliable graph node indexing.
- `mermaid_id` (function) - Sanitizes file or module names to produce valid, URL-safe identifiers required by Mermaid.js node definitions.
- `get_module` (function) - Extracts the logical module name from a file path by stripping extensions and directory separators.

## Key Concepts

- **Directed Dependency Graph**: Abstracts cross-language import relationships into a unified DiGraph, enabling consistent traversal and analysis regardless of source language.
- **PageRank File Ranking**: Uses iterative power iteration to score files by inbound dependency density, allowing the system to automatically prioritize high-impact modules for documentation.
- **Language-Specific Import Resolution**: Employs regex patterns tailored to each language to accurately map import statements to normalized project file paths, bridging syntactic differences into a common graph schema.
- **Strongly Connected Components (SCC)**: Detects circular dependencies that indicate tight coupling or architectural anti-patterns, preventing infinite loops during recursive documentation generation.
- **Mermaid Visualization Export**: Converts internal graph topology into standard Mermaid syntax, enabling static wiki rendering without requiring client-side JavaScript execution.

## Internal Relationships

- `crates/graph/src/lib.rs` → `repowiki_core::models::ProjectContext`: Consumes structured project metadata to seed the dependency graph with file paths and language hints.
- `crates/graph/src/lib.rs` → `petgraph`: Leverages DiGraph for efficient node/edge storage and tarjan_scc for strongly connected component detection.
- `crates/graph/src/lib.rs` → `regex`: Uses compiled patterns to parse language-specific import syntax during initial graph construction.
- `graph module` → `documentation_pipeline`: Feeds ranked file lists, dependency maps, and Mermaid diagrams into the LLM documentation generator to prioritize coverage and validate architecture.
