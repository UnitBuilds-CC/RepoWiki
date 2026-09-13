# core

> Provide foundational configuration management and strongly-typed data models for project scanning, static analysis indexing, and structured wiki generation.

The core module serves as the central data and configuration layer. It abstracts runtime settings, resolves LLM provider aliases, and persists user preferences to eliminate hardcoded dependencies. Simultaneously, it defines the complete schema for static analysis outputs and wiki documents. By enforcing strict typing across scan reports, symbol indexes, and architectural diagrams, the module ensures downstream components consume structured data directly. This eliminates unstructured LLM parsing, reduces token consumption, and guarantees consistent documentation output.

## Files

### `crates/core/Cargo.toml`

Package manifest defining dependencies, primarily serde for data serialization required by config and models.

- `dependencies` (toml_table) - Declares serde and other runtime requirements for the crate.

### `crates/core/src/config.rs`

Manages application configuration lifecycle, path resolution, and model aliasing.

- `config_dir` (function) - Computes the standard directory path for storing application configuration.
- `config_file` (function) - Constructs the full path to the active configuration JSON file.
- `model_aliases` (function) - Returns a static mapping of shorthand provider names to their full endpoint strings.
- `resolve_model` (function) - Resolves a user-provided model name against aliases and returns the canonical string.
- `Config` (struct) - Holds runtime parameters (model, api_key, concurrency, limits) with built-in defaults.
- `ConfigFile::load` (method) - Deserializes and validates configuration from disk into a Config struct.
- `ConfigFile::save` (method) - Serializes current configuration state back to disk atomically.

### `crates/core/src/lib.rs`

Crate entry point that establishes the public API boundary by re-exporting core types.

- `mod config` (module_declaration) - Exposes configuration module to external consumers.
- `mod models` (module_declaration) - Exposes domain data structures for indexing and wiki generation.

### `crates/core/src/models.rs`

Defines all domain data structures for static analysis, indexing, and wiki document generation.

- `FileInfo` (struct) - Represents a scanned file's metadata, content preview, and classification flags.
- `ScanReport` (struct) - Aggregates scan outcomes including kept candidates, oversized/binary drops, and skipped directories.
- `ProjectContext` (struct) - Holds root-level project metadata, file tree, and coverage statistics.
- `IndexedFile` (struct) - Stores parsed metrics, symbol lists, import/export graphs, and content hashes for a single file.
- `IndexedSymbol` (struct) - Captures detailed symbol attributes including visibility, parameters, return types, and call targets.
- `ProjectIndex` (struct) - Root container for the entire codebase index, linking modules, call graphs, and symbol lookups.
- `WikiData` (struct) - Top-level schema for the final generated wiki, aggregating overview, modules, architecture, and reading guides.
- `ArchitectureDiagram` (struct) - Structures component relationships, data flows, and Mermaid diagram definitions.
- `ReadingGuide` (struct) - Defines sequential learning steps, time estimates, and contextual tips for new developers.
- `SymbolKind` (enum) - Type-safe categorization of code elements (function, class, variable, etc.).
- `Visibility` (enum) - Encapsulates access modifiers (public, private, protected) for accurate indexing.

## Key Concepts

- **Structured Indexing**: Raw code is transformed into typed graphs (ProjectIndex, CallEdge, IndexedSymbol) rather than raw text. This allows downstream LLM calls to query precise relationships and metadata, drastically cutting token usage and preventing hallucination.
- **Constrained Wiki Generation**: Pre-defined schemas (WikiData, ModuleDoc, ArchitectureDiagram) force LLM outputs into exact shapes. This eliminates verbose prose, enforces consistency, and ensures every generated page contains only actionable information.
- **Configuration Abstraction**: Path resolution and model aliasing decouple provider selection from business logic. Users configure endpoints once, and the system routes requests transparently without conditional branching in core pipelines.

## Internal Relationships

- `config.rs` → `lib.rs`: lib.rs re-exports Config and resolution functions, making them accessible to CLI and web handlers.
- `models.rs` → `lib.rs`: lib.rs re-exports all domain structs, establishing the shared contract for indexing pipelines and documentation generators.
- `config.rs` → `models.rs`: Configuration parameters (max_file_size, concurrency, language) directly dictate filtering thresholds and parsing behavior when populating ScanReport and IndexedFile instances.
