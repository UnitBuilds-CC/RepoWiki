# export

> Converts internally analyzed wiki structures into multiple static output formats for distribution and consumption.

The export module transforms the project's internal Wiki representation—built from LLM-indexed codebase analysis—into human-readable and machine-parseable documentation artifacts. It supports three primary output targets: Markdown (optimized for version control and GitHub wikis), JSON (for programmatic consumption or static site generators), and HTML (for direct web viewing). The module emphasizes efficiency through incremental file writing, state/version tracking to skip unchanged content, and robust sanitization to prevent XSS or broken links. Submodules handle format-specific rendering, serialization, and site scaffolding, all orchestrated through a unified Wiki interface.

## Files

### `crates/export/Cargo.toml`

Package manifest defining dependencies, crate metadata, and build configuration for the export module.

### `crates/export/src/html.rs`

Renders the wiki as a static HTML site. Handles markdown-to-HTML conversion, link resolution, path normalization, and HTML escaping. Uses embedded templates for consistent layout and secure rendering.

- `export_html` (function) - Main entry point; orchestrates HTML generation and writes files to disk.
- `markdown_to_html` (function) - Parses markdown strings into HTML, resolving cross-page links.
- `inline_md` (function) - Processes inline markdown snippets within larger documents.
- `normalize_path` (function) - Sanitizes and standardizes relative paths for HTML anchors.
- `html_escape` (function) - Prevents XSS by escaping special characters.
- `HTML_TEMPLATE` (constant) - Base HTML layout template.
- `page` (constant) - Template for individual documentation pages.

### `crates/export/src/json_export.rs`

Serializes the wiki into a structured JSON payload. Defines data models for pages and navigation trees to enable programmatic consumption or integration with static site generators.

- `JsonExport` (struct) - Root container holding project name, pages, and sidebar data.
- `PageEntry` (struct) - Represents a single documentation page with metadata and content.
- `SidebarEntry` (struct) - Models navigation nodes with hierarchical children.
- `export_json` (function) - Traverses the wiki and writes serialized JSON to the target path.
- `serialize_sidebar` (function) - Recursively flattens the sidebar tree into a JSON-compatible array.

### `crates/export/src/lib.rs`

Module root; re-exports public functions and types from submodules for external use.

### `crates/export/src/markdown.rs`

Generates version-controlled Markdown files. Implements incremental export logic to minimize I/O and preserve git history. Handles sidebar/README generation and internal link cleanup.

- `STATE_FILENAME` (constant) - Filename for persisting export state across runs.
- `STATE_VERSION` (constant) - Schema version for state file compatibility checks.
- `ExportSummary` (struct) - Tracks counts of written, kept, and removed files.
- `StateFile` (struct) - Persists export metadata including model, language, and page states.
- `PageState` (struct) - Caches input hashes per page to detect modifications.
- `load_state` (function) - Reads and deserializes the previous export state.
- `save_state` (function) - Writes updated export metadata to disk.
- `export_markdown` (function) - Iterates pages, applies change detection, and writes markdown files.
- `eq` (method) - Compares two PageState instances for equality.
- `write_if_changed` (function) - Performs atomic writes only when content differs from existing files.
- `sidebar_text` (function) - Generates navigation markdown blocks.
- `readme_text` (function) - Creates overview markdown for the repository root.
- `strip_md_from_internal_links` (function) - Removes raw markdown syntax from links to ensure renderer compatibility.

### `crates/export/src/site.rs`

Scaffolds a basic HTML landing page for the exported site to serve as a navigation entry point.

- `INDEX_HTML` (constant) - Template string for the site entry point.
- `write_site_loader` (function) - Writes the index file to the target directory.

## Key Concepts

- **Incremental Export**: Tracks file hashes and state versions to skip rewriting unchanged content, reducing I/O overhead and preserving clean git diffs.
- **Format Abstraction**: A unified Wiki input is routed to format-specific renderers (MD, JSON, HTML) without coupling the core analysis engine to output concerns.
- **Sanitization & Link Resolution**: Normalizes paths, escapes HTML, and strips raw markdown syntax from links to ensure cross-platform compatibility and security.
- **State Persistence**: Uses versioned state files to maintain export continuity across tool runs, enabling reliable delta updates and preventing redundant processing.

## Internal Relationships

- `lib.rs` → `html.rs`: Aggregates and exposes HTML export functionality to consumers.
- `lib.rs` → `json_export.rs`: Aggregates and exposes JSON serialization functionality to consumers.
- `lib.rs` → `markdown.rs`: Aggregates and exposes Markdown generation and state management functionality to consumers.
- `lib.rs` → `site.rs`: Aggregates and exposes site scaffolding functionality to consumers.
- `html.rs` → `repowiki_wiki::Wiki`: Consumes the central Wiki struct to extract pages, titles, and content for HTML rendering.
- `markdown.rs` → `repowiki_wiki::Wiki`: Consumes the central Wiki struct to extract pages, titles, and content for Markdown generation.
- `markdown.rs` → `repowiki_cache::content_hash`: Uses content hashing to compute checksums for incremental export optimization.
- `site.rs` → `html.rs`: Complements HTML export by providing the root navigation page that ties the generated site together.
- `json_export.rs` → `repowiki_wiki::Wiki`: Consumes the central Wiki struct and SidebarItem definitions to serialize navigation and page data.
