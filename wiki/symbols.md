# Symbol Index

213 symbols across 13 modules.

## Const

### [cache](cache)

- [`DEFAULT_TTL`](cache) - Sets the default expiration duration for cached entries in seconds.

## Constant

### [export](export)

- [`HTML_TEMPLATE`](export) - Base HTML layout template.
- [`INDEX_HTML`](export) - Template string for the site entry point.
- [`STATE_FILENAME`](export) - Filename for persisting export state across runs.
- [`STATE_VERSION`](export) - Schema version for state file compatibility checks.
- [`page`](export) - Template for individual documentation pages.

### [ingest](ingest)

- [`MAX_REPO_SIZE_MB`](ingest) - Hard limit on repository size in megabytes to prevent resource exhaustion and control LLM costs.
- [`git_url_regex`](ingest) - Compiled regex pattern used to validate and extract owner, repository, and branch components from Git URLs.

### [scanner](scanner)

- [`CODE_LANGS`](scanner) - Mapping of file extensions to recognized programming languages.
- [`CONFIG_FILES`](scanner) - Names of standard configuration files to prioritize during indexing.
- [`ENTRYPOINT_DIRS`](scanner) - Directory names typically containing main execution logic.
- [`ENTRYPOINT_NAMES`](scanner) - Common filenames indicating application entry points or routers.
- [`MINIFIED_SOURCE_EXTS`](scanner) - Extensions commonly associated with compressed frontend assets.
- [`SENSITIVE_NAMES`](scanner) - Filenames containing secrets or credentials that are always filtered out.
- [`SKIP_DIRS`](scanner) - List of directory names automatically excluded from traversal (e.g., node_modules, .git).
- [`SKIP_EXTS`](scanner) - File extensions treated as non-source or build artifacts.

## Enum

### [cache](cache)

- [`CacheError`](cache) - Defines error types for database operations, IO failures, and cache mismatches.

### [cli](cli)

- [`Commands`](cli) - Maps CLI subcommands to their corresponding handler functions.

### [core](core)

- [`SymbolKind`](core) - Type-safe categorization of code elements (function, class, variable, etc.).
- [`Visibility`](core) - Encapsulates access modifiers (public, private, protected) for accurate indexing.

### [llm](llm)

- [`LLMError`](llm) - Standardized error type for API failures and parsing issues.

## Function

### [analyzer](analyzer)

- [`analyze_one_module`](analyzer) - Executes the actual LLM call for a single module, applying caching, prompt templating, and JSON extraction.
- [`build_key_files_context`](analyzer) - Formats critical entry-point files into a concise string for prompt injection, reducing raw code bloat.
- [`build_module_summary`](analyzer) - Extracts and formats structural metadata from the project index to provide lightweight context for LLM queries.

### [cache](cache)

- [`cache_dir`](cache) - Returns the standard directory path where the cache database and related files are stored.
- [`content_hash`](cache) - Computes a SHA-256 hex digest of a string to enable deterministic deduplication of identical code/content.
- [`default_db_path`](cache) - Constructs the full file path to the SQLite database within the cache directory.
- [`now_secs`](cache) - Returns the current Unix timestamp as a float for TTL calculations.

### [cli](cli)

- [`cmd_cache_clear`](cli) - Purges the local caching layer storing intermediate analysis and LLM responses.
- [`cmd_chat`](cli) - Starts an interactive REPL session with the LLM, injecting retrieved code context into prompts.
- [`cmd_config`](cli) - Manages persistent configuration, including model resolution and credential storage.
- [`cmd_index`](cli) - Builds or refreshes the semantic index used for RAG-based code retrieval.
- [`cmd_map`](cli) - Generates and outputs a dependency visualization or summary for a specified project path.
- [`cmd_scan`](cli) - Scans a directory tree, collects file metadata, and produces a structured scan report.
- [`cmd_serve`](cli) - Initializes and runs an asynchronous HTTP server to expose the generated wiki and API endpoints.
- [`run_analysis`](cli) - Orchestrates the core workflow: loads project context, formats prompts, queries the LLM, and constructs wiki pages.

### [core](core)

- [`config_dir`](core) - Computes the standard directory path for storing application configuration.
- [`config_file`](core) - Constructs the full path to the active configuration JSON file.
- [`model_aliases`](core) - Returns a static mapping of shorthand provider names to their full endpoint strings.
- [`resolve_model`](core) - Resolves a user-provided model name against aliases and returns the canonical string.

### [export](export)

- [`export_html`](export) - Main entry point; orchestrates HTML generation and writes files to disk.
- [`export_json`](export) - Traverses the wiki and writes serialized JSON to the target path.
- [`export_markdown`](export) - Iterates pages, applies change detection, and writes markdown files.
- [`html_escape`](export) - Prevents XSS by escaping special characters.
- [`inline_md`](export) - Processes inline markdown snippets within larger documents.
- [`load_state`](export) - Reads and deserializes the previous export state.
- [`markdown_to_html`](export) - Parses markdown strings into HTML, resolving cross-page links.
- [`normalize_path`](export) - Sanitizes and standardizes relative paths for HTML anchors.
- [`readme_text`](export) - Creates overview markdown for the repository root.
- [`save_state`](export) - Writes updated export metadata to disk.
- [`serialize_sidebar`](export) - Recursively flattens the sidebar tree into a JSON-compatible array.
- [`sidebar_text`](export) - Generates navigation markdown blocks.
- [`strip_md_from_internal_links`](export) - Removes raw markdown syntax from links to ensure renderer compatibility.
- [`write_if_changed`](export) - Performs atomic writes only when content differs from existing files.
- [`write_site_loader`](export) - Writes the index file to the target directory.

### [frontend](frontend)

- [`escapeHtml`](frontend) - Strips dangerous characters from markdown content to prevent XSS attacks.
- [`getFileContent`](frontend) - Downloads raw source file content referenced within the documentation.
- [`getHeaders`](frontend) - Attaches authentication tokens and content-type headers to outgoing requests.
- [`getPage`](frontend) - Retrieves the full markdown content for a specific wiki page by ID.
- [`getWiki`](frontend) - Fetches the complete documentation tree structure for a given project.
- [`handleScan`](frontend) - Validates form inputs, calls the scan API, updates global store with progress, and redirects upon completion.
- [`handleSend`](frontend) - Processes user input, appends it to the store, triggers the streaming API call, and manages loading states.
- [`markdownToHtml`](frontend) - Converts standard markdown segments into sanitized HTML for rendering.
- [`scanProject`](frontend) - Triggers the backend indexing pipeline and returns the resulting project identifier.
- [`splitMermaid`](frontend) - Parses input markdown string and separates standard text from fenced Mermaid code blocks.
- [`streamChat`](frontend) - Sends user queries to the LLM and yields incremental response chunks via stream.
- [`streamScanProgress`](frontend) - Establishes a streaming connection to monitor real-time indexing status and logs.

### [graph](graph)

- [`get_module`](graph) - Derives a logical module name from a file path.
- [`import_patterns`](graph) - Returns language-specific regex patterns for extracting import statements.
- [`mermaid_id`](graph) - Escapes and formats file names into valid Mermaid node identifiers.
- [`normalize_path`](graph) - Sanitizes and standardizes file paths to ensure consistent node identification.
- [`pagerank_power_iteration`](graph) - Implements the iterative PageRank algorithm to converge on stable importance scores.
- [`resolve_import`](graph) - Matches an import statement against known patterns and resolves it to a target file path.
- [`resolve_python_module`](graph) - Specialized resolver for Python import paths, handling relative imports and module-to-file mapping.

### [index](index)

- [`build_call_graph`](index) - Aggregates flow data from all files into a unified call relationship structure.
- [`build_candidates`](index) - Generates filesystem and namespace permutations for an import path to maximize resolution success.
- [`build_index`](index) - Entry point that iterates over files, applies caching, and returns a complete ProjectIndex.
- [`build_outgoing`](index) - Generates CallEdge records mapping caller functions to their invoked callees.
- [`build_symbol_index`](index) - Creates a reverse lookup map from symbol names to file paths for fast resolution.
- [`compute_metrics`](index) - Calculates file-level statistics including cyclomatic complexity and line counts for prioritization.
- [`extract_go`](index) - Scans Go files for packages, structs, methods, interfaces, and import declarations.
- [`extract_java`](index) - Processes Java classes, methods, fields, annotations, and package imports with visibility detection.
- [`extract_module_path`](index) - Strips syntax prefixes and qualifiers to isolate the canonical module identifier.
- [`extract_python`](index) - Parses Python files to collect classes, functions, imports, docstrings, and method calls using indentation-aware scanning.
- [`extract_rust`](index) - Extracts Rust modules, structs, impl blocks, functions, macros, and trait implementations via pattern matching.
- [`extract_symbols`](index) - Public dispatcher that routes raw source content to the correct language extractor based on detected syntax.
- [`extract_typescript`](index) - Captures TS interfaces, classes, methods, enums, and ES module imports using brace/indent tracking.
- [`format_module_context`](index) - Serializes a ModuleIndex into a compact, LLM-friendly string containing summaries, symbols, and edges.
- [`group_into_module_indices`](index) - Clusters related IndexedFiles into logical ModuleIndex units based on directory and naming conventions.
- [`index_one_file`](index) - Handles single-file processing: checks cache, runs extractor, resolves imports, and computes metrics.
- [`resolve_imports`](index) - Batch resolver that normalizes all import paths across the indexed project.
- [`resolve_one`](index) - Attempts to match a single import statement against candidate paths using language-specific heuristics.
- [`trace_flow`](index) - Analyzes extracted call sites to build a graph of execution dependencies between symbols.

### [ingest](ingest)

- [`authenticated_clone_url`](ingest) - Injects resolved credentials into the clone URL to enable secure access to restricted repositories.
- [`clone_dir`](ingest) - Creates an isolated temporary directory for safely cloning repositories without polluting the host filesystem.
- [`clone_url`](ingest) - Constructs the base git clone command string for unauthenticated access.
- [`dir_size_mb`](ingest) - Recursively calculates the total size of a cloned directory in megabytes for size validation.
- [`guess_project_name`](ingest) - Derives a readable project identifier from the root directory path or contained file names.
- [`ingest_github`](ingest) - Main orchestrator that clones, validates size, extracts metadata, and returns a standardized ProjectContext and ScanReport.
- [`ingest_local`](ingest) - Orchestrates local directory scanning, delegates to repowiki_scanner, and returns a unified ProjectContext and ScanReport.
- [`parse_git_url`](ingest) - Extracts and validates the owner, repository name, and branch/tag from a raw Git URL string.
- [`remove_dir_all`](ingest) - Safely deletes the temporary clone directory and all its contents after processing completes.
- [`resolve_token`](ingest) - Securely retrieves a GitHub personal access token from environment variables for private repository access.

### [llm](llm)

- [`build_architecture_prompt`](llm) - Constructs prompts focused on cross-module relationships and system design.
- [`build_module_prompt`](llm) - Creates prompts for analyzing individual modules or directories.
- [`build_overview_prompt`](llm) - Generates system and user messages for high-level codebase summarization.
- [`build_reading_guide_prompt`](llm) - Produces prompts that generate navigational instructions for developers.
- [`complete`](llm) - Sends a prompt sequence and returns a single complete response.
- [`extract_json`](llm) - Applies regex filtering to isolate valid JSON blocks from raw LLM text.
- [`resolve_api_base`](llm) - Determines the correct endpoint URL based on model and key heuristics.
- [`resolve_model_name`](llm) - Normalizes model identifiers to match provider expectations.
- [`stream`](llm) - Sends a prompt sequence and yields an async iterator of response chunks.

### [rag](rag)

- [`cosine_similarity`](rag) - Computes dot product of normalized TF vectors to determine semantic alignment between query and indexed chunks.
- [`format_context`](rag) - Converts retrieved Chunk slices into a delimited string optimized for direct injection into LLM system prompts.
- [`index_fingerprint`](rag) - Generates a deterministic SHA-256 hash from project metadata to detect structural or content changes.
- [`load_or_build_index`](rag) - Validates project state against a cached SHA-256 fingerprint. Loads existing index if unchanged, otherwise triggers a full rebuild.
- [`split_into_chunks`](rag) - Divides file contents into fixed-length line ranges, attaching file path and line boundaries to each segment.
- [`tokenize`](rag) - Splits raw text into alphanumeric tokens using regex, stripping punctuation and normalizing case for consistent vector mapping.

### [scanner](scanner)

- [`build_file_tree`](scanner) - Formats a sorted slice of FileInfo objects into a hierarchical string representation for wiki rendering.
- [`detect_language`](scanner) - Maps a file path's extension to a human-readable language identifier using CODE_LANGS.
- [`glob_match`](scanner) - Public wrapper that delegates to the inner byte-level glob evaluator.
- [`glob_match_inner`](scanner) - Low-level recursive parser that matches glob patterns against raw byte slices without allocating intermediate strings.
- [`has_skipped_suffix`](scanner) - Checks if a path ends with any extension in SKIP_EXTS.
- [`is_binary`](scanner) - Inspects file headers for magic bytes to determine if a file is binary and should be skipped.
- [`is_entrypoint`](scanner) - Determines if a path corresponds to a known entrypoint file or directory.
- [`is_sensitive_name`](scanner) - Verifies if a filename matches any pattern in SENSITIVE_NAMES.
- [`looks_minified_source`](scanner) - Applies content heuristics to detect highly compressed or obfuscated source code.
- [`scan_directory`](scanner) - Main entry point that walks a directory tree, applies IgnoreRules and heuristics, collects metadata, and returns a ScanReport.
- [`sort_key`](scanner) - Generates a tuple priority key to order entrypoints and configurations above regular source files.

### [server](server)

- [`chat`](server) - POST endpoint that queries the RAG index, builds a prompt using repowiki_llm, and streams the LLM's token-by-token response.
- [`create_app`](server) - Constructs and returns the root Axum Router, wiring routes and injecting AppState.
- [`get_graph`](server) - GET endpoint returning the serialized dependency graph for visualization.
- [`get_page`](server) - GET endpoint fetching rendered markdown content for a specific wiki page.
- [`get_wiki`](server) - GET endpoint returning the top-level wiki structure and sidebar items.
- [`run_scan`](server) - Background executor that coordinates ingestion, graph building, and wiki generation, updating AppState upon completion.
- [`start_scan`](server) - POST endpoint that validates input, generates a project ID, and spawns a background task to run the scan.
- [`stream_status`](server) - GET endpoint that establishes an SSE connection to stream real-time scan progress to the client.

## Interface

### [frontend](frontend)

- [`ChatMessage`](frontend) - Represents a complete chat turn including role, content, and optional references.
- [`ChatReference`](frontend) - Tracks context snippets or source links attached to AI chat responses.
- [`ChatTurn`](frontend) - Schema representing a single message exchange in the AI chat session.
- [`ContentPart`](frontend) - Discriminated union type distinguishing between standard markdown text and Mermaid diagram blocks.
- [`PageMeta`](frontend) - Lightweight metadata for wiki pages used in navigation and previews.
- [`ProjectInfo`](frontend) - Metadata returned after successful project scan completion.
- [`Props`](frontend) - Defines expected props for diagram rendering including content and container dimensions.
- [`ScanRequest`](frontend) - Schema for initiating a new codebase indexing job.
- [`SidebarItem`](frontend) - Individual node definition for sidebar navigation components.
- [`WikiPage`](frontend) - Full document payload containing markdown content and associated metadata.
- [`WikiStore`](frontend) - Defines the shape of the global state slice including selectors, setters, and initialization logic.
- [`WikiStructure`](frontend) - Hierarchical representation of the generated documentation tree.

## Method

### [analyzer](analyzer)

- [`analyze`](analyzer) - Top-level workflow coordinator that triggers overview, module, architecture, and reading guide generation sequentially.
- [`analyze_modules`](analyzer) - Iterates through the project index, delegating individual module analysis to analyze_one_module while respecting concurrency limits.
- [`generate_architecture`](analyzer) - Constructs system-level design documentation by traversing the dependency graph and injecting it into architecture prompts.
- [`generate_overview`](analyzer) - Produces a high-level project summary using aggregated index data and overview-specific prompts.
- [`generate_reading_guide`](analyzer) - Outputs a curated sequence of modules and files to help developers navigate the codebase logically based on dependencies.
- [`new`](analyzer) - Constructor initializing the analyzer with required dependencies and configurable concurrency limits.

### [cache](cache)

- [`Cache::clear`](cache) - Drops all cached entries and resets the database, returning the number of removed rows.
- [`Cache::get`](cache) - Retrieves a cached JSON value by key, returning None if expired or missing.
- [`Cache::get_default_ttl`](cache) - Fetches a cached value using DEFAULT_TTL for expiration checks.
- [`Cache::load_project`](cache) - Loads a previously saved project document or index state by ID.
- [`Cache::open`](cache) - Initializes or opens the SQLite database at the specified path, creating tables if necessary.
- [`Cache::put`](cache) - Stores a JSON value under a key with a custom TTL, overwriting existing entries.
- [`Cache::save_project`](cache) - Persists a complete project document or index state under a project-scoped key.

### [core](core)

- [`ConfigFile::load`](core) - Deserializes and validates configuration from disk into a Config struct.
- [`ConfigFile::save`](core) - Serializes current configuration state back to disk atomically.

### [export](export)

- [`eq`](export) - Compares two PageState instances for equality.

### [graph](graph)

- [`build_from_project`](graph) - Parses ProjectContext to extract imports using language-specific regex patterns and constructs the dependency graph.
- [`edges`](graph) - Extracts all directed edge pairs from the graph.
- [`find_circular_dependencies`](graph) - Uses Tarjan's strongly connected components algorithm to detect and return cycles up to a specified limit.
- [`find_isolated_files`](graph) - Locates nodes with no incoming or outgoing edges, indicating orphaned or standalone code.
- [`get_core_files`](graph) - Returns the top N highest-ranked files based on PageRank scores.
- [`get_entry_points`](graph) - Identifies nodes with zero incoming edges, representing public APIs or binaries.
- [`get_module_dependencies`](graph) - Aggregates dependencies by logical module rather than individual files.
- [`nodes`](graph) - Extracts all node identifiers from the graph.
- [`rank_files`](graph) - Computes PageRank scores for all nodes to quantify file importance and connectivity.
- [`to_mermaid`](graph) - Serializes the graph into Mermaid.js flowchart syntax for visual inspection.

### [rag](rag)

- [`index`](rag) - Processes a ProjectContext, splits source files into bounded line chunks, computes TF-IDF statistics, and populates internal vectors.
- [`retrieve`](rag) - Tokenizes a query, calculates cosine similarity against stored TF vectors, and returns the top-k highest-scoring Chunks.

### [scanner](scanner)

- [`from_root`](scanner) - Initializes IgnoreRules by loading patterns from a specified directory root.
- [`matches`](scanner) - Evaluates a relative path and directory flag against stored patterns to return true if the path should be ignored.

## Module_declaration

### [core](core)

- [`mod config`](core) - Exposes configuration module to external consumers.
- [`mod models`](core) - Exposes domain data structures for indexing and wiki generation.

## Struct

### [analyzer](analyzer)

- [`Analyzer`](analyzer) - Stateful orchestrator holding references to the LLM client, cache, concurrency semaphore, language, and output prefix.

### [cache](cache)

- [`Cache`](cache) - Wraps a rusqlite Connection and provides methods for reading, writing, and managing cached data.

### [cli](cli)

- [`Cli`](cli) - Root structure defining the top-level CLI schema parsed by clap.

### [core](core)

- [`ArchitectureDiagram`](core) - Structures component relationships, data flows, and Mermaid diagram definitions.
- [`Config`](core) - Holds runtime parameters (model, api_key, concurrency, limits) with built-in defaults.
- [`FileInfo`](core) - Represents a scanned file's metadata, content preview, and classification flags.
- [`IndexedFile`](core) - Stores parsed metrics, symbol lists, import/export graphs, and content hashes for a single file.
- [`IndexedSymbol`](core) - Captures detailed symbol attributes including visibility, parameters, return types, and call targets.
- [`ProjectContext`](core) - Holds root-level project metadata, file tree, and coverage statistics.
- [`ProjectIndex`](core) - Root container for the entire codebase index, linking modules, call graphs, and symbol lookups.
- [`ReadingGuide`](core) - Defines sequential learning steps, time estimates, and contextual tips for new developers.
- [`ScanReport`](core) - Aggregates scan outcomes including kept candidates, oversized/binary drops, and skipped directories.
- [`WikiData`](core) - Top-level schema for the final generated wiki, aggregating overview, modules, architecture, and reading guides.

### [export](export)

- [`ExportSummary`](export) - Tracks counts of written, kept, and removed files.
- [`JsonExport`](export) - Root container holding project name, pages, and sidebar data.
- [`PageEntry`](export) - Represents a single documentation page with metadata and content.
- [`PageState`](export) - Caches input hashes per page to detect modifications.
- [`SidebarEntry`](export) - Models navigation nodes with hierarchical children.
- [`StateFile`](export) - Persists export metadata including model, language, and page states.

### [graph](graph)

- [`DependencyGraph`](graph) - Wraps a petgraph DiGraph to represent files as nodes and imports as directed edges.

### [llm](llm)

- [`ChatMessage`](llm) - Represents role/content pairs used in conversation history and prompts.
- [`LLMClient`](llm) - Core client wrapper that holds configuration, tracks cumulative token usage/cost, and dispatches requests.

### [rag](rag)

- [`IndexPayload`](rag) - Serde-compatible DTO wrapping chunks, IDF map, and TF vectors for safe JSON serialization and deserialization.
- [`SimpleRAG`](rag) - Core orchestrator storing indexed chunks, IDF weights, and TF vectors. Manages indexing, persistence, and query execution.

### [scanner](scanner)

- [`IgnoreRules`](scanner) - Holds compiled filter patterns used to decide if a path should be skipped.

### [server](server)

- [`AppState`](server) - Global application state holding the cache and a mutex-protected map of active projects.
- [`ChatRequest`](server) - Schema for chat queries containing the question and conversation history.
- [`ProjectInfo`](server) - Response payload reporting scan status, file counts, line counts, and errors.
- [`ProjectState`](server) - Per-project runtime context storing metadata, wiki instance, RAG index, and progress tracking.
- [`ScanRequest`](server) - Schema for triggering a new documentation scan with path, URL, language, and model configuration.

## Toml_table

### [core](core)

- [`dependencies`](core) - Declares serde and other runtime requirements for the crate.

## Type_alias

### [analyzer](analyzer)

- [`ProgressFn`](analyzer) - Callback signature for reporting incremental analysis progress to CLI or web interfaces.
