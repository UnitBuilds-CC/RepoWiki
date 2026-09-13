# export

> Converts the internal Wiki data model into static documentation formats (Markdown, JSON, HTML) with incremental write optimization.

The export module transforms the centralized Wiki representation into distributable static assets. It supports three primary output formats while implementing a state-tracking mechanism to skip unchanged files, reducing disk I/O and build times. Each formatter consumes the same Wiki struct but applies format-specific serialization, markdown parsing, and template rendering logic.

## Files

### `crates/export/src/html.rs`

Generates a complete HTML documentation site from the Wiki model.

- `export_html` (function) - Orchestrates the full HTML generation pipeline, writing rendered pages to the specified output path.
- `markdown_to_html` (function) - Converts markdown strings to HTML, resolving internal wiki links and handling cross-page references.
- `inline_md` (function) - Processes inline markdown elements within HTML context, preserving formatting while sanitizing output.
- `normalize_path` (function) - Standardizes relative paths for consistent link resolution across generated pages.
- `html_escape` (function) - Sanitizes raw text to prevent XSS and ensure valid HTML output.
- `HTML_TEMPLATE` (constant) - Base HTML structure used for all generated documentation pages.
- `page` (constant) - Page-specific HTML template fragment containing head, body, and script placeholders.

### `crates/export/src/json_export.rs`

Serializes the Wiki structure and content into a flat JSON format for external tooling or API consumption.

- `JsonExport` (struct) - Top-level JSON container holding project metadata, serialized pages, and navigation sidebar.
- `PageEntry` (struct) - Represents a single wiki page with its ID, title, markdown content, parent relationship, and sort order.
- `SidebarEntry` (struct) - Mirrors the Wiki sidebar hierarchy for JSON-based navigation structures.
- `export_json` (function) - Traverses the Wiki model, serializes pages and sidebar, and writes the final JSON file.
- `serialize_sidebar` (function) - Recursively flattens the sidebar tree into a JSON-compatible vector of entries.

### `crates/export/src/markdown.rs`

Generates standard Markdown files with incremental write optimization to avoid redundant disk operations.

- `ExportSummary` (struct) - Tracks metrics for the export run: files written, kept unchanged, and removed.
- `StateFile` (struct) - Persists version, model metadata, and per-page input hashes to enable change detection.
- `PageState` (struct) - Stores the hash of inputs for a specific page to determine if regeneration is needed.
- `load_state` (function) - Reads the previous export state file to compare against current wiki contents.
- `save_state` (function) - Writes updated state and page hashes after a successful export run.
- `export_markdown` (function) - Main orchestrator that iterates through wiki pages, checks state, and writes only changed files.
- `write_if_changed` (function) - Compares new content against existing files and disk state, skipping writes if identical.
- `sidebar_text` (function) - Generates the markdown-formatted navigation sidebar.
- `readme_text` (function) - Generates the root README.md file from the wiki root node.

### `crates/export/src/site.rs`

Creates a minimal HTML landing page to serve as the documentation site entry point.

- `INDEX_HTML` (constant) - Static HTML string for the site loader/root index page.
- `write_site_loader` (function) - Writes the INDEX_HTML file to the target directory, optionally customizing the title.

### `crates/export/src/lib.rs`

Module entry point that re-exports public APIs from submodules for clean external usage.

## Key Concepts

- **Incremental Export**: Uses content hashing and a persisted state file to detect unchanged pages, skipping disk writes to speed up rebuilds and reduce I/O overhead.
- **Unified Wiki Consumption**: All formatters accept the same repowiki_wiki::Wiki struct, ensuring a single source of truth regardless of output format.
- **Link & Path Normalization**: Consistent path resolution and HTML escaping across formats prevent broken references and security vulnerabilities in generated docs.
- **Template-Driven Rendering**: HTML output uses predefined templates (HTML_TEMPLATE, page) to maintain consistent styling and structure without runtime dependencies.

## Internal Relationships

- `lib.rs` → `html.rs`: lib.rs re-exports export_html and related types for direct consumption by CLI/web handlers.
- `lib.rs` → `json_export.rs`: lib.rs exposes export_json and serialization structs to support programmatic wiki dumps.
- `lib.rs` → `markdown.rs`: lib.rs aggregates markdown export functions and state management utilities.
- `lib.rs` → `site.rs`: lib.rs provides write_site_loader to bootstrap the generated documentation directory.
- `markdown.rs` → `html.rs`: Both modules parse and transform markdown content, but html.rs handles browser-safe rendering while markdown.rs focuses on source fidelity.
- `html.rs` → `markdown.rs`: html.rs relies on similar path normalization and escaping logic patterns found in markdown.rs for consistent link handling.
