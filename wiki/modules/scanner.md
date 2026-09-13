# scanner

> Provides cost-efficient filesystem ingestion by traversing repositories, applying intelligent filtering rules, and classifying files for downstream wiki generation.

The scanner module serves as the primary ingestion layer for the documentation generator. It recursively walks target directories, applies pattern-based and heuristic filters to discard binaries, build artifacts, minified assets, and sensitive files, and classifies remaining source files by language and architectural role. This pre-processing step drastically reduces the volume of data forwarded to LLMs, lowering token costs while preserving structural context through entrypoint detection and ordered file tree construction. It implements a custom glob matcher to handle ignore rules efficiently and outputs structured metadata compatible with the core indexing pipeline.

## Files

### `crates/scanner/Cargo.toml`

Defines the scanner crate, declares runtime dependencies (walkdir, tracing, repowiki_core), and sets compilation targets.

### `crates/scanner/src/ignore_rules.rs`

Implements a custom pattern-matching engine to evaluate whether files or directories should be excluded during traversal.

- `IgnoreRules` (struct) - Holds compiled filter patterns used to decide if a path should be skipped.
- `from_root` (method) - Initializes IgnoreRules by loading patterns from a specified directory root.
- `matches` (method) - Evaluates a relative path and directory flag against stored patterns to return true if the path should be ignored.
- `glob_match` (function) - Public wrapper that delegates to the inner byte-level glob evaluator.
- `glob_match_inner` (function) - Low-level recursive parser that matches glob patterns against raw byte slices without allocating intermediate strings.

### `crates/scanner/src/lib.rs`

Module root that re-exports public symbols from ignore_rules and scan to establish a clean crate boundary.

### `crates/scanner/src/scan.rs`

Orchestrates directory traversal, file classification, binary/minified detection, and structured tree generation for indexing.

- `SKIP_DIRS` (constant) - List of directory names automatically excluded from traversal (e.g., node_modules, .git).
- `SKIP_EXTS` (constant) - File extensions treated as non-source or build artifacts.
- `SENSITIVE_NAMES` (constant) - Filenames containing secrets or credentials that are always filtered out.
- `MINIFIED_SOURCE_EXTS` (constant) - Extensions commonly associated with compressed frontend assets.
- `CODE_LANGS` (constant) - Mapping of file extensions to recognized programming languages.
- `CONFIG_FILES` (constant) - Names of standard configuration files to prioritize during indexing.
- `ENTRYPOINT_NAMES` (constant) - Common filenames indicating application entry points or routers.
- `ENTRYPOINT_DIRS` (constant) - Directory names typically containing main execution logic.
- `detect_language` (function) - Maps a file path's extension to a human-readable language identifier using CODE_LANGS.
- `is_binary` (function) - Inspects file headers for magic bytes to determine if a file is binary and should be skipped.
- `has_skipped_suffix` (function) - Checks if a path ends with any extension in SKIP_EXTS.
- `is_sensitive_name` (function) - Verifies if a filename matches any pattern in SENSITIVE_NAMES.
- `looks_minified_source` (function) - Applies content heuristics to detect highly compressed or obfuscated source code.
- `is_entrypoint` (function) - Determines if a path corresponds to a known entrypoint file or directory.
- `build_file_tree` (function) - Formats a sorted slice of FileInfo objects into a hierarchical string representation for wiki rendering.
- `scan_directory` (function) - Main entry point that walks a directory tree, applies IgnoreRules and heuristics, collects metadata, and returns a ScanReport.
- `sort_key` (function) - Generates a tuple priority key to order entrypoints and configurations above regular source files.

## Key Concepts

- **Cost-Aware Filtering**: 
- **Heuristic Classification**: 
- **Entrypoint Prioritization**: 
- **Custom Glob Matching**: 

## Internal Relationships

- `crates/scanner/src/scan.rs` → `crates/scanner/src/ignore_rules.rs`: scan.rs instantiates IgnoreRules and calls matches() during WalkDir iteration to prune irrelevant paths before processing.
- `crates/scanner/src/scan.rs` → `repowiki_core::models`: scan.rs constructs FileInfo and ScanReport structs from repowiki_core to pass structured results to the indexing pipeline.
- `crates/scanner/src/lib.rs` → `crates/scanner/src/scan.rs`: lib.rs re-exports scan.rs public functions to expose the scanning API to external crates.
- `crates/scanner/src/lib.rs` → `crates/scanner/src/ignore_rules.rs`: lib.rs re-exports IgnoreRules and its methods for direct use by callers outside the scanner crate.
