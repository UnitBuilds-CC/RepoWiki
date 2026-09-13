# Symbol Index

209 symbols across 13 modules.

## Const

### [cache](modules/cache.md)

- [`DEFAULT_TTL`](modules/cache.md) - Sets the default expiration duration for cached entries in seconds.

## Constant

### [export](modules/export.md)

- [`HTML_TEMPLATE`](modules/export.md) - Base HTML structure used for all generated documentation pages.
- [`INDEX_HTML`](modules/export.md) - Static HTML string for the site loader/root index page.
- [`page`](modules/export.md) - Page-specific HTML template fragment containing head, body, and script placeholders.

### [ingest](modules/ingest.md)

- [`MAX_REPO_SIZE_MB`](modules/ingest.md) - Hard limit on repository size in megabytes to prevent resource exhaustion and control LLM costs.
- [`git_url_regex`](modules/ingest.md) - Compiled regex pattern used to validate and extract owner, repository, and branch components from Git URLs.

### [scanner](modules/scanner.md)

- [`CODE_LANGS`](modules/scanner.md) - Mapping of file extensions to recognized programming languages.
- [`CONFIG_FILES`](modules/scanner.md) - Names of standard configuration files to prioritize during indexing.
- [`ENTRYPOINT_DIRS`](modules/scanner.md) - Directory names typically containing main execution logic.
- [`ENTRYPOINT_NAMES`](modules/scanner.md) - Common filenames indicating application entry points or routers.
- [`MINIFIED_SOURCE_EXTS`](modules/scanner.md) - Extensions commonly associated with compressed frontend assets.
- [`SENSITIVE_NAMES`](modules/scanner.md) - Filenames containing secrets or credentials that are always filtered out.
- [`SKIP_DIRS`](modules/scanner.md) - List of directory names automatically excluded from traversal (e.g., node_modules, .git).
- [`SKIP_EXTS`](modules/scanner.md) - File extensions treated as non-source or build artifacts.

## Enum

### [cache](modules/cache.md)

- [`CacheError`](modules/cache.md) - Defines error types for database operations, IO failures, and cache mismatches.

### [cli](modules/cli.md)

- [`Commands`](modules/cli.md) - Defines available subcommands: map, scan, serve, cache-clear, index, chat, and config.

### [core](modules/core.md)

- [`SymbolKind`](modules/core.md) - Type-safe categorization of code elements (function, class, variable, etc.).
- [`Visibility`](modules/core.md) - Encapsulates access modifiers (public, private, protected) for accurate indexing.

### [llm](modules/llm.md)

- [`LLMError`](modules/llm.md) - Standardized error type for API failures and parsing issues.

## Function

### [analyzer](modules/analyzer.md)

- [`analyze_one_module`](modules/analyzer.md) - Executes the actual LLM call for a single module, applying caching, prompt templating, and JSON extraction.
- [`build_key_files_context`](modules/analyzer.md) - Formats critical entry-point files into a concise string for prompt injection, reducing raw code bloat.
- [`build_module_summary`](modules/analyzer.md) - Extracts and formats structural metadata from the project index to provide lightweight context for LLM queries.

### [cache](modules/cache.md)

- [`cache_dir`](modules/cache.md) - Returns the standard directory path where the cache database and related files are stored.
- [`content_hash`](modules/cache.md) - Computes a SHA-256 hex digest of a string to enable deterministic deduplication of identical code/content.
- [`default_db_path`](modules/cache.md) - Constructs the full file path to the SQLite database within the cache directory.
- [`now_secs`](modules/cache.md) - Returns the current Unix timestamp as a float for TTL calculations.

### [cli](modules/cli.md)

- [`cmd_cache_clear`](modules/cli.md) - Deletes cached analysis results and LLM responses to force a complete regeneration.
- [`cmd_chat`](modules/cli.md) - Runs an interactive REPL-style session that queries the LLM using indexed context and formatted prompts.
- [`cmd_config`](modules/cli.md) - Manages LLM provider settings and model selection, resolving configurations at runtime.
- [`cmd_index`](modules/cli.md) - Builds or loads the structured code index, which maps symbols and relationships to minimize tokens sent to the LLM.
- [`cmd_map`](modules/cli.md) - Generates a visual dependency graph of the target repository and outputs it in formats like Mermaid or JSON.
- [`cmd_scan`](modules/cli.md) - Traverses the filesystem to produce a structured ScanReport containing file paths, languages, and metadata.
- [`cmd_serve`](modules/cli.md) - Initializes an Actix web server with shared AppState, exposing HTTP endpoints for browser-based wiki generation and chat.
- [`run_analysis`](modules/cli.md) - Orchestrates the core pipeline: scans the repo, builds the cost-optimized index, feeds context to the Analyzer and WikiBuilder, and generates documentation.

### [core](modules/core.md)

- [`config_dir`](modules/core.md) - Computes the standard directory path for storing application configuration.
- [`config_file`](modules/core.md) - Constructs the full path to the active configuration JSON file.
- [`model_aliases`](modules/core.md) - Returns a static mapping of shorthand provider names to their full endpoint strings.
- [`resolve_model`](modules/core.md) - Resolves a user-provided model name against aliases and returns the canonical string.

### [export](modules/export.md)

- [`export_html`](modules/export.md) - Orchestrates the full HTML generation pipeline, writing rendered pages to the specified output path.
- [`export_json`](modules/export.md) - Traverses the Wiki model, serializes pages and sidebar, and writes the final JSON file.
- [`export_markdown`](modules/export.md) - Main orchestrator that iterates through wiki pages, checks state, and writes only changed files.
- [`html_escape`](modules/export.md) - Sanitizes raw text to prevent XSS and ensure valid HTML output.
- [`inline_md`](modules/export.md) - Processes inline markdown elements within HTML context, preserving formatting while sanitizing output.
- [`load_state`](modules/export.md) - Reads the previous export state file to compare against current wiki contents.
- [`markdown_to_html`](modules/export.md) - Converts markdown strings to HTML, resolving internal wiki links and handling cross-page references.
- [`normalize_path`](modules/export.md) - Standardizes relative paths for consistent link resolution across generated pages.
- [`readme_text`](modules/export.md) - Generates the root README.md file from the wiki root node.
- [`save_state`](modules/export.md) - Writes updated state and page hashes after a successful export run.
- [`serialize_sidebar`](modules/export.md) - Recursively flattens the sidebar tree into a JSON-compatible vector of entries.
- [`sidebar_text`](modules/export.md) - Generates the markdown-formatted navigation sidebar.
- [`write_if_changed`](modules/export.md) - Compares new content against existing files and disk state, skipping writes if identical.
- [`write_site_loader`](modules/export.md) - Writes the INDEX_HTML file to the target directory, optionally customizing the title.

### [frontend](modules/frontend.md)

- [`escapeHtml`](modules/frontend.md) - Strips dangerous characters from markdown content to prevent XSS attacks.
- [`getFileContent`](modules/frontend.md) - Downloads raw source file content referenced within the documentation.
- [`getHeaders`](modules/frontend.md) - Attaches authentication tokens and content-type headers to outgoing requests.
- [`getPage`](modules/frontend.md) - Retrieves the full markdown content for a specific wiki page by ID.
- [`getWiki`](modules/frontend.md) - Fetches the complete documentation tree structure for a given project.
- [`handleScan`](modules/frontend.md) - Validates form inputs, calls the scan API, updates global store with progress, and redirects upon completion.
- [`handleSend`](modules/frontend.md) - Processes user input, appends it to the store, triggers the streaming API call, and manages loading states.
- [`markdownToHtml`](modules/frontend.md) - Converts standard markdown segments into sanitized HTML for rendering.
- [`scanProject`](modules/frontend.md) - Triggers the backend indexing pipeline and returns the resulting project identifier.
- [`splitMermaid`](modules/frontend.md) - Parses input markdown string and separates standard text from fenced Mermaid code blocks.
- [`streamChat`](modules/frontend.md) - Sends user queries to the LLM and yields incremental response chunks via stream.
- [`streamScanProgress`](modules/frontend.md) - Establishes a streaming connection to monitor real-time indexing status and logs.

### [graph](modules/graph.md)

- [`get_module`](modules/graph.md) - Derives a logical module name from a file path.
- [`import_patterns`](modules/graph.md) - Returns language-specific regex patterns for extracting import statements.
- [`mermaid_id`](modules/graph.md) - Escapes and formats file names into valid Mermaid node identifiers.
- [`normalize_path`](modules/graph.md) - Sanitizes and standardizes file paths to ensure consistent node identification.
- [`pagerank_power_iteration`](modules/graph.md) - Implements the iterative PageRank algorithm to converge on stable importance scores.
- [`resolve_import`](modules/graph.md) - Matches an import statement against known patterns and resolves it to a target file path.
- [`resolve_python_module`](modules/graph.md) - Specialized resolver for Python import paths, handling relative imports and module-to-file mapping.

### [index](modules/index.md)

- [`build_call_graph`](modules/index.md) - Aggregates flow data from all files into a unified call relationship structure.
- [`build_candidates`](modules/index.md) - Generates filesystem and namespace permutations for an import path to maximize resolution success.
- [`build_index`](modules/index.md) - Entry point that iterates over files, applies caching, and returns a complete ProjectIndex.
- [`build_outgoing`](modules/index.md) - Generates CallEdge records mapping caller functions to their invoked callees.
- [`build_symbol_index`](modules/index.md) - Creates a reverse lookup map from symbol names to file paths for fast resolution.
- [`compute_metrics`](modules/index.md) - Calculates file-level statistics including cyclomatic complexity and line counts for prioritization.
- [`extract_go`](modules/index.md) - Scans Go files for packages, structs, methods, interfaces, and import declarations.
- [`extract_java`](modules/index.md) - Processes Java classes, methods, fields, annotations, and package imports with visibility detection.
- [`extract_module_path`](modules/index.md) - Strips syntax prefixes and qualifiers to isolate the canonical module identifier.
- [`extract_python`](modules/index.md) - Parses Python files to collect classes, functions, imports, docstrings, and method calls using indentation-aware scanning.
- [`extract_rust`](modules/index.md) - Extracts Rust modules, structs, impl blocks, functions, macros, and trait implementations via pattern matching.
- [`extract_symbols`](modules/index.md) - Public dispatcher that routes raw source content to the correct language extractor based on detected syntax.
- [`extract_typescript`](modules/index.md) - Captures TS interfaces, classes, methods, enums, and ES module imports using brace/indent tracking.
- [`format_module_context`](modules/index.md) - Serializes a ModuleIndex into a compact, LLM-friendly string containing summaries, symbols, and edges.
- [`group_into_module_indices`](modules/index.md) - Clusters related IndexedFiles into logical ModuleIndex units based on directory and naming conventions.
- [`index_one_file`](modules/index.md) - Handles single-file processing: checks cache, runs extractor, resolves imports, and computes metrics.
- [`resolve_imports`](modules/index.md) - Batch resolver that normalizes all import paths across the indexed project.
- [`resolve_one`](modules/index.md) - Attempts to match a single import statement against candidate paths using language-specific heuristics.
- [`trace_flow`](modules/index.md) - Analyzes extracted call sites to build a graph of execution dependencies between symbols.

### [ingest](modules/ingest.md)

- [`authenticated_clone_url`](modules/ingest.md) - Injects resolved credentials into the clone URL to enable secure access to restricted repositories.
- [`clone_dir`](modules/ingest.md) - Creates an isolated temporary directory for safely cloning repositories without polluting the host filesystem.
- [`clone_url`](modules/ingest.md) - Constructs the base git clone command string for unauthenticated access.
- [`dir_size_mb`](modules/ingest.md) - Recursively calculates the total size of a cloned directory in megabytes for size validation.
- [`guess_project_name`](modules/ingest.md) - Derives a readable project identifier from the root directory path or contained file names.
- [`ingest_github`](modules/ingest.md) - Main orchestrator that clones, validates size, extracts metadata, and returns a standardized ProjectContext and ScanReport.
- [`ingest_local`](modules/ingest.md) - Orchestrates local directory scanning, delegates to repowiki_scanner, and returns a unified ProjectContext and ScanReport.
- [`parse_git_url`](modules/ingest.md) - Extracts and validates the owner, repository name, and branch/tag from a raw Git URL string.
- [`remove_dir_all`](modules/ingest.md) - Safely deletes the temporary clone directory and all its contents after processing completes.
- [`resolve_token`](modules/ingest.md) - Securely retrieves a GitHub personal access token from environment variables for private repository access.

### [llm](modules/llm.md)

- [`build_architecture_prompt`](modules/llm.md) - Constructs prompts focused on cross-module relationships and system design.
- [`build_module_prompt`](modules/llm.md) - Creates prompts for analyzing individual modules or directories.
- [`build_overview_prompt`](modules/llm.md) - Generates system and user messages for high-level codebase summarization.
- [`build_reading_guide_prompt`](modules/llm.md) - Produces prompts that generate navigational instructions for developers.
- [`complete`](modules/llm.md) - Sends a prompt sequence and returns a single complete response.
- [`extract_json`](modules/llm.md) - Applies regex filtering to isolate valid JSON blocks from raw LLM text.
- [`resolve_api_base`](modules/llm.md) - Determines the correct endpoint URL based on model and key heuristics.
- [`resolve_model_name`](modules/llm.md) - Normalizes model identifiers to match provider expectations.
- [`stream`](modules/llm.md) - Sends a prompt sequence and yields an async iterator of response chunks.

### [rag](modules/rag.md)

- [`cosine_similarity`](modules/rag.md) - Computes dot product of normalized TF vectors to determine semantic alignment between query and indexed chunks.
- [`format_context`](modules/rag.md) - Converts retrieved Chunk slices into a delimited string optimized for direct injection into LLM system prompts.
- [`index_fingerprint`](modules/rag.md) - Generates a deterministic SHA-256 hash from project metadata to detect structural or content changes.
- [`load_or_build_index`](modules/rag.md) - Validates project state against a cached SHA-256 fingerprint. Loads existing index if unchanged, otherwise triggers a full rebuild.
- [`split_into_chunks`](modules/rag.md) - Divides file contents into fixed-length line ranges, attaching file path and line boundaries to each segment.
- [`tokenize`](modules/rag.md) - Splits raw text into alphanumeric tokens using regex, stripping punctuation and normalizing case for consistent vector mapping.

### [scanner](modules/scanner.md)

- [`build_file_tree`](modules/scanner.md) - Formats a sorted slice of FileInfo objects into a hierarchical string representation for wiki rendering.
- [`detect_language`](modules/scanner.md) - Maps a file path's extension to a human-readable language identifier using CODE_LANGS.
- [`glob_match`](modules/scanner.md) - Public wrapper that delegates to the inner byte-level glob evaluator.
- [`glob_match_inner`](modules/scanner.md) - Low-level recursive parser that matches glob patterns against raw byte slices without allocating intermediate strings.
- [`has_skipped_suffix`](modules/scanner.md) - Checks if a path ends with any extension in SKIP_EXTS.
- [`is_binary`](modules/scanner.md) - Inspects file headers for magic bytes to determine if a file is binary and should be skipped.
- [`is_entrypoint`](modules/scanner.md) - Determines if a path corresponds to a known entrypoint file or directory.
- [`is_sensitive_name`](modules/scanner.md) - Verifies if a filename matches any pattern in SENSITIVE_NAMES.
- [`looks_minified_source`](modules/scanner.md) - Applies content heuristics to detect highly compressed or obfuscated source code.
- [`scan_directory`](modules/scanner.md) - Main entry point that walks a directory tree, applies IgnoreRules and heuristics, collects metadata, and returns a ScanReport.
- [`sort_key`](modules/scanner.md) - Generates a tuple priority key to order entrypoints and configurations above regular source files.

### [server](modules/server.md)

- [`chat`](modules/server.md) - POST endpoint that queries the RAG index, builds a prompt using repowiki_llm, and streams the LLM's token-by-token response.
- [`create_app`](modules/server.md) - Constructs and returns the root Axum Router, wiring routes and injecting AppState.
- [`get_graph`](modules/server.md) - GET endpoint returning the serialized dependency graph for visualization.
- [`get_page`](modules/server.md) - GET endpoint fetching rendered markdown content for a specific wiki page.
- [`get_wiki`](modules/server.md) - GET endpoint returning the top-level wiki structure and sidebar items.
- [`run_scan`](modules/server.md) - Background executor that coordinates ingestion, graph building, and wiki generation, updating AppState upon completion.
- [`start_scan`](modules/server.md) - POST endpoint that validates input, generates a project ID, and spawns a background task to run the scan.
- [`stream_status`](modules/server.md) - GET endpoint that establishes an SSE connection to stream real-time scan progress to the client.

## Interface

### [frontend](modules/frontend.md)

- [`ChatMessage`](modules/frontend.md) - Represents a complete chat turn including role, content, and optional references.
- [`ChatReference`](modules/frontend.md) - Tracks context snippets or source links attached to AI chat responses.
- [`ChatTurn`](modules/frontend.md) - Schema representing a single message exchange in the AI chat session.
- [`ContentPart`](modules/frontend.md) - Discriminated union type distinguishing between standard markdown text and Mermaid diagram blocks.
- [`PageMeta`](modules/frontend.md) - Lightweight metadata for wiki pages used in navigation and previews.
- [`ProjectInfo`](modules/frontend.md) - Metadata returned after successful project scan completion.
- [`Props`](modules/frontend.md) - Defines expected props for diagram rendering including content and container dimensions.
- [`ScanRequest`](modules/frontend.md) - Schema for initiating a new codebase indexing job.
- [`SidebarItem`](modules/frontend.md) - Individual node definition for sidebar navigation components.
- [`WikiPage`](modules/frontend.md) - Full document payload containing markdown content and associated metadata.
- [`WikiStore`](modules/frontend.md) - Defines the shape of the global state slice including selectors, setters, and initialization logic.
- [`WikiStructure`](modules/frontend.md) - Hierarchical representation of the generated documentation tree.

## Method

### [analyzer](modules/analyzer.md)

- [`analyze`](modules/analyzer.md) - Top-level workflow coordinator that triggers overview, module, architecture, and reading guide generation sequentially.
- [`analyze_modules`](modules/analyzer.md) - Iterates through the project index, delegating individual module analysis to analyze_one_module while respecting concurrency limits.
- [`generate_architecture`](modules/analyzer.md) - Constructs system-level design documentation by traversing the dependency graph and injecting it into architecture prompts.
- [`generate_overview`](modules/analyzer.md) - Produces a high-level project summary using aggregated index data and overview-specific prompts.
- [`generate_reading_guide`](modules/analyzer.md) - Outputs a curated sequence of modules and files to help developers navigate the codebase logically based on dependencies.
- [`new`](modules/analyzer.md) - Constructor initializing the analyzer with required dependencies and configurable concurrency limits.

### [cache](modules/cache.md)

- [`Cache::clear`](modules/cache.md) - Drops all cached entries and resets the database, returning the number of removed rows.
- [`Cache::get`](modules/cache.md) - Retrieves a cached JSON value by key, returning None if expired or missing.
- [`Cache::get_default_ttl`](modules/cache.md) - Fetches a cached value using DEFAULT_TTL for expiration checks.
- [`Cache::load_project`](modules/cache.md) - Loads a previously saved project document or index state by ID.
- [`Cache::open`](modules/cache.md) - Initializes or opens the SQLite database at the specified path, creating tables if necessary.
- [`Cache::put`](modules/cache.md) - Stores a JSON value under a key with a custom TTL, overwriting existing entries.
- [`Cache::save_project`](modules/cache.md) - Persists a complete project document or index state under a project-scoped key.

### [core](modules/core.md)

- [`ConfigFile::load`](modules/core.md) - Deserializes and validates configuration from disk into a Config struct.
- [`ConfigFile::save`](modules/core.md) - Serializes current configuration state back to disk atomically.

### [graph](modules/graph.md)

- [`build_from_project`](modules/graph.md) - Parses ProjectContext to extract imports using language-specific regex patterns and constructs the dependency graph.
- [`edges`](modules/graph.md) - Extracts all directed edge pairs from the graph.
- [`find_circular_dependencies`](modules/graph.md) - Uses Tarjan's strongly connected components algorithm to detect and return cycles up to a specified limit.
- [`find_isolated_files`](modules/graph.md) - Locates nodes with no incoming or outgoing edges, indicating orphaned or standalone code.
- [`get_core_files`](modules/graph.md) - Returns the top N highest-ranked files based on PageRank scores.
- [`get_entry_points`](modules/graph.md) - Identifies nodes with zero incoming edges, representing public APIs or binaries.
- [`get_module_dependencies`](modules/graph.md) - Aggregates dependencies by logical module rather than individual files.
- [`nodes`](modules/graph.md) - Extracts all node identifiers from the graph.
- [`rank_files`](modules/graph.md) - Computes PageRank scores for all nodes to quantify file importance and connectivity.
- [`to_mermaid`](modules/graph.md) - Serializes the graph into Mermaid.js flowchart syntax for visual inspection.

### [rag](modules/rag.md)

- [`index`](modules/rag.md) - Processes a ProjectContext, splits source files into bounded line chunks, computes TF-IDF statistics, and populates internal vectors.
- [`retrieve`](modules/rag.md) - Tokenizes a query, calculates cosine similarity against stored TF vectors, and returns the top-k highest-scoring Chunks.

### [scanner](modules/scanner.md)

- [`from_root`](modules/scanner.md) - Initializes IgnoreRules by loading patterns from a specified directory root.
- [`matches`](modules/scanner.md) - Evaluates a relative path and directory flag against stored patterns to return true if the path should be ignored.

## Module_declaration

### [core](modules/core.md)

- [`mod config`](modules/core.md) - Exposes configuration module to external consumers.
- [`mod models`](modules/core.md) - Exposes domain data structures for indexing and wiki generation.

## Struct

### [analyzer](modules/analyzer.md)

- [`Analyzer`](modules/analyzer.md) - Stateful orchestrator holding references to the LLM client, cache, concurrency semaphore, language, and output prefix.

### [cache](modules/cache.md)

- [`Cache`](modules/cache.md) - Wraps a rusqlite Connection and provides methods for reading, writing, and managing cached data.

### [cli](modules/cli.md)

- [`Cli`](modules/cli.md) - Root clap struct defining the top-level CLI interface and routing to subcommands.

### [core](modules/core.md)

- [`ArchitectureDiagram`](modules/core.md) - Structures component relationships, data flows, and Mermaid diagram definitions.
- [`Config`](modules/core.md) - Holds runtime parameters (model, api_key, concurrency, limits) with built-in defaults.
- [`FileInfo`](modules/core.md) - Represents a scanned file's metadata, content preview, and classification flags.
- [`IndexedFile`](modules/core.md) - Stores parsed metrics, symbol lists, import/export graphs, and content hashes for a single file.
- [`IndexedSymbol`](modules/core.md) - Captures detailed symbol attributes including visibility, parameters, return types, and call targets.
- [`ProjectContext`](modules/core.md) - Holds root-level project metadata, file tree, and coverage statistics.
- [`ProjectIndex`](modules/core.md) - Root container for the entire codebase index, linking modules, call graphs, and symbol lookups.
- [`ReadingGuide`](modules/core.md) - Defines sequential learning steps, time estimates, and contextual tips for new developers.
- [`ScanReport`](modules/core.md) - Aggregates scan outcomes including kept candidates, oversized/binary drops, and skipped directories.
- [`WikiData`](modules/core.md) - Top-level schema for the final generated wiki, aggregating overview, modules, architecture, and reading guides.

### [export](modules/export.md)

- [`ExportSummary`](modules/export.md) - Tracks metrics for the export run: files written, kept unchanged, and removed.
- [`JsonExport`](modules/export.md) - Top-level JSON container holding project metadata, serialized pages, and navigation sidebar.
- [`PageEntry`](modules/export.md) - Represents a single wiki page with its ID, title, markdown content, parent relationship, and sort order.
- [`PageState`](modules/export.md) - Stores the hash of inputs for a specific page to determine if regeneration is needed.
- [`SidebarEntry`](modules/export.md) - Mirrors the Wiki sidebar hierarchy for JSON-based navigation structures.
- [`StateFile`](modules/export.md) - Persists version, model metadata, and per-page input hashes to enable change detection.

### [graph](modules/graph.md)

- [`DependencyGraph`](modules/graph.md) - Wraps a petgraph DiGraph to represent files as nodes and imports as directed edges.

### [llm](modules/llm.md)

- [`ChatMessage`](modules/llm.md) - Represents role/content pairs used in conversation history and prompts.
- [`LLMClient`](modules/llm.md) - Core client wrapper that holds configuration, tracks cumulative token usage/cost, and dispatches requests.

### [rag](modules/rag.md)

- [`IndexPayload`](modules/rag.md) - Serde-compatible DTO wrapping chunks, IDF map, and TF vectors for safe JSON serialization and deserialization.
- [`SimpleRAG`](modules/rag.md) - Core orchestrator storing indexed chunks, IDF weights, and TF vectors. Manages indexing, persistence, and query execution.

### [scanner](modules/scanner.md)

- [`IgnoreRules`](modules/scanner.md) - Holds compiled filter patterns used to decide if a path should be skipped.

### [server](modules/server.md)

- [`AppState`](modules/server.md) - Global application state holding the cache and a mutex-protected map of active projects.
- [`ChatRequest`](modules/server.md) - Schema for chat queries containing the question and conversation history.
- [`ProjectInfo`](modules/server.md) - Response payload reporting scan status, file counts, line counts, and errors.
- [`ProjectState`](modules/server.md) - Per-project runtime context storing metadata, wiki instance, RAG index, and progress tracking.
- [`ScanRequest`](modules/server.md) - Schema for triggering a new documentation scan with path, URL, language, and model configuration.

## Toml_table

### [core](modules/core.md)

- [`dependencies`](modules/core.md) - Declares serde and other runtime requirements for the crate.

## Type_alias

### [analyzer](modules/analyzer.md)

- [`ProgressFn`](modules/analyzer.md) - Callback signature for reporting incremental analysis progress to CLI or web interfaces.
