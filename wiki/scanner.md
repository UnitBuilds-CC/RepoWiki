# scanner

> Traverses and filters local codebases to extract structured file metadata and content for LLM-based documentation generation.

The scanner module walks a repository directory tree to identify, classify, and filter source files before they are passed to the LLM pipeline. It applies heuristic and pattern-based rules to exclude binaries, minified assets, sensitive configurations, build artifacts, and dependency directories. By detecting programming languages, identifying entry points, and sorting files by relevance, it ensures that only high-value, parseable code consumes the LLM context window. The module outputs a sorted list of file metadata and a concatenated file tree string ready for downstream processing.

## Files

### `crates/scanner/Cargo.toml`

Defines crate metadata and declares runtime dependencies like walkdir and tracing.

### `crates/scanner/src/ignore_rules.rs`

Implements pattern-matching logic to determine which files and directories should be excluded from the scan.

- `IgnoreRules` (struct) - Holds a collection of glob-style patterns used to filter out unwanted paths.
- `from_root` (function) - Constructs an IgnoreRules instance by loading standard exclusion patterns relative to the repository root.
- `matches` (function) - Evaluates whether a given relative path and directory flag should be skipped based on loaded patterns.
- `glob_match` (function) - Lightweight custom glob matcher that compares a pattern against a file path without external dependencies.

### `crates/scanner/src/lib.rs`

Serves as the module entry point, re-exporting public types and functions from submodules.

### `crates/scanner/src/scan.rs`

Orchestrates filesystem traversal, applies classification and filtering rules, and assembles the final scan report.

- `SKIP_DIRS` (constant) - Hardcoded list of directory names to exclude (e.g., .git, node_modules, target).
- `SKIP_EXTS` (constant) - File extensions filtered out to avoid parsing binaries or generated assets.
- `SENSITIVE_NAMES` (constant) - Filenames containing secrets, credentials, or private keys that must never be scanned.
- `MINIFIED_SOURCE_EXTS` (constant) - Extensions for compressed frontend bundles that lack readable structure.
- `CODE_LANGS` (constant) - Supported programming language extensions mapped for detection purposes.
- `CONFIG_FILES` (constant) - Standard configuration filenames used to identify project setup files.
- `ENTRYPOINT_NAMES` (constant) - Filenames recognized as application or library entry points.
- `ENTRYPOINT_DIRS` (constant) - Directory names treated as primary source roots.
- `lang_map` (function) - Returns a static lookup table mapping file extensions to canonical language identifiers.
- `detect_language` (function) - Resolves the programming language of a file by checking its extension against lang_map.
- `is_binary` (function) - Inspects raw file bytes to detect binary formats and prevent them from being parsed as text.
- `has_skipped_suffix` (function) - Checks if a path ends with an extension defined in SKIP_EXTS.
- `is_sensitive_name` (function) - Verifies if a filename matches any pattern in SENSITIVE_NAMES.
- `looks_minified_source` (function) - Heuristic check to identify compressed JavaScript/TypeScript files that lack readable structure.
- `is_entrypoint` (function) - Determines if a file matches known root configuration or application startup filenames.
- `build_file_tree` (function) - Concatenates the contents of filtered FileInfo objects into a single delimited string for LLM context injection.
- `scan_directory` (function) - Main traversal function that uses WalkDir, applies IgnoreRules, classifies files, and returns a ScanReport.
- `sort_key` (function) - Generates a sort tuple that prioritizes entry points and sorts remaining files alphabetically for deterministic output.

## Key Concepts

- **Noise Reduction**: Filters out binaries, minified code, secrets, and dependency folders to prevent wasting LLM tokens on unparseable or irrelevant content.
- **Entry Point Prioritization**: Ranks configuration and startup files above general source files to ensure the LLM processes architectural context first.
- **Deterministic Ordering**: Uses a stable sort key to guarantee consistent file ordering across runs, which improves LLM prompt reliability and caching.
- **Context Assembly**: Flattens multiple filtered files into a single structured string, optimizing memory usage and token boundaries for downstream AI processing.

## Internal Relationships

- `crates/scanner/src/scan.rs` → `crates/scanner/src/ignore_rules.rs`: scan.rs instantiates IgnoreRules and calls its matches method during WalkDir iteration to prune irrelevant branches early.
- `crates/scanner/src/scan.rs` → `repowiki_core::models`: Consumes FileInfo and ScanReport structs to format the scanned data into the application's expected domain model.
- `crates/scanner/src/lib.rs` → `crates/scanner/src/scan.rs`: Re-exports scan_directory and related public APIs so external crates can invoke the scanner directly.
