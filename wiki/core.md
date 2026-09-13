# core

> Central data contract and configuration manager for the documentation pipeline.

The core module establishes the foundational data structures and runtime configuration required to scan, index, and document arbitrary codebases. It decouples the parsing and LLM generation phases by providing strongly-typed representations of project metadata, file contents, symbol indices, and architectural relationships. Configuration handling abstracts away environment-specific API credentials, model routing, and resource constraints, ensuring consistent behavior across CLI and web interfaces.

## Files

### `crates/core/src/config.rs`

Manages persistent application settings, API credentials, model routing, and processing constraints. Provides defaults and deserializes/serializes user preferences to ensure reproducible documentation generation.

- `Config` (struct) - Runtime settings container holding API keys, base URLs, model selection, concurrency limits, and file processing thresholds.
- `ConfigFile` (struct) - Disk-persisted format for configuration, containing only essential fields like model, api_key, api_base, and language.
- `resolve_model` (function) - Maps shorthand alias strings to full provider/model identifiers based on a static lookup table.
- `load` (method) - Deserializes configuration from disk into a Config instance, applying defaults for missing fields.
- `save` (method) - Serializes current configuration state back to disk, enabling persistent user preferences.

### `crates/core/src/models.rs`

Defines the complete domain schema for the documentation pipeline. Structures represent raw scan results, indexed code artifacts, architectural mappings, and the final wiki payload. Enables deterministic serialization for LLM prompts and downstream rendering.

- `ScanReport` (struct) - Categorizes discovered files by size, type, priority, and filtering status during initial traversal.
- `ProjectContext` (struct) - Holds repository root, name, and aggregated file tree used to drive LLM context windows.
- `IndexedFile` (struct) - Parsed representation of a single source file containing metrics, symbols, imports, exports, and content hash.
- `IndexedSymbol` (struct) - Detailed breakdown of individual code elements including visibility, parameters, return types, decorators, and call targets.
- `ProjectIndex` (struct) - Aggregated view containing all modules, call graph edges, and global symbol index for cross-file analysis.
- `ArchitectureDiagram` (struct) - Structured representation of system design including components, sequence flows, and Mermaid diagram definitions.
- `ReadingGuide` (struct) - Curated step-by-step onboarding plan with time estimates and file references for new developers.
- `WikiData` (struct) - Final assembled documentation payload composing overview, modules, architecture, and reading guide into a unified output.
- `Visibility` (enum) - Classifies symbol accessibility (public, private, protected) for accurate documentation scoping.
- `SymbolKind` (enum) - Identifies code element types (function, class, variable, module, etc.) for proper categorization.

### `crates/core/src/lib.rs`

Module root. Re-exports public types and functions from config and models to establish a clean public API for downstream crates.

## Key Concepts

- **Domain-Driven Data Contracts**: Strongly-typed structs replace ad-hoc dictionaries, ensuring type safety and predictable serialization when passing data between the scanner, LLM orchestrator, and wiki renderer.
- **Progressive Indexing**: Raw files are first classified (ScanReport), then parsed into rich artifacts (IndexedFile/IndexedSymbol), and finally aggregated (ProjectIndex) to build call graphs and architectural maps before LLM consumption.
- **Configuration Abstraction**: Separates environment-sensitive settings (API keys, base URLs, model aliases) from business logic, allowing dynamic routing and resource tuning without code changes.
- **Structured Output Assembly**: WikiData enforces a rigid hierarchy (overview -> modules -> architecture -> reading guide) that guarantees consistent markdown/wiki formatting regardless of source language or project size.

## Internal Relationships

- `config.rs` → `models.rs`: Configuration values (max_file_size, concurrency, language) directly filter and control the population of ScanReport and IndexedFile structures during the scanning phase.
- `models.rs` → `models.rs`: ProjectIndex aggregates ModuleIndex and CallEdge data, which feeds into ArchitectureDiagram and ReadingGuide generation. WikiData composes ProjectOverview, ModuleDoc, and ReadingGuide into the final output.
- `lib.rs` → `config.rs`: Acts as a facade, exposing configuration utilities to the rest of the workspace without leaking internal paths.
- `lib.rs` → `models.rs`: Exports the entire domain schema to downstream crates, enabling uniform type usage across the CLI and web interface.
