# Symbol Index

207 symbols across 13 modules.

## Const

### [cache](cache)

- [`DEFAULT_TTL`](cache) - Sets the default expiration duration in seconds for cached entries.

## Constant

### [export](export)

- [`HTML_TEMPLATE`](export) - Base HTML skeleton used for wrapping exported pages.
- [`INDEX_HTML`](export) - Static HTML string defining the layout and script injection for the site loader.
- [`page`](export) - Page-specific HTML template containing structural markup and placeholder injection points.

### [rag](rag)

- [`INDEX_DIR_NAME`](rag) - Defines the standard directory name used to store cached retrieval indices on disk.

### [scanner](scanner)

- [`CODE_LANGS`](scanner) - Supported programming language extensions mapped for detection purposes.
- [`CONFIG_FILES`](scanner) - Standard configuration filenames used to identify project setup files.
- [`ENTRYPOINT_DIRS`](scanner) - Directory names treated as primary source roots.
- [`ENTRYPOINT_NAMES`](scanner) - Filenames recognized as application or library entry points.
- [`MINIFIED_SOURCE_EXTS`](scanner) - Extensions for compressed frontend bundles that lack readable structure.
- [`SENSITIVE_NAMES`](scanner) - Filenames containing secrets, credentials, or private keys that must never be scanned.
- [`SKIP_DIRS`](scanner) - Hardcoded list of directory names to exclude (e.g., .git, node_modules, target).
- [`SKIP_EXTS`](scanner) - File extensions filtered out to avoid parsing binaries or generated assets.

## Enum

### [cache](cache)

- [`CacheError`](cache) - Defines errors for database connection failures, query execution issues, and I/O problems.

### [cli](cli)

- [`Commands`](cli) - Defines all available CLI subcommands including map, scan, serve, cache-clear, index, chat, and config.
- [`ConfigAction`](cli) - Enumerates configuration management operations like setting or retrieving API keys.
- [`MapFormat`](cli) - Specifies output serialization formats for dependency graphs.
- [`ScanFormat`](cli) - Controls how scan reports are serialized for terminal or file output.

### [core](core)

- [`SymbolKind`](core) - Identifies code element types (function, class, variable, module, etc.) for proper categorization.
- [`Visibility`](core) - Classifies symbol accessibility (public, private, protected) for accurate documentation scoping.

## Function

### [analyzer](analyzer)

- [`analyze_one_module`](analyzer) - Handles single-module analysis: checks cache, constructs prompt, calls LLM, parses JSON response, and stores result.
- [`build_key_files_context`](analyzer) - Extracts and formats critical project files into a concise string for LLM context injection.
- [`build_module_summary`](analyzer) - Condenses the project index into a structured summary to reduce token usage during module analysis.

### [cache](cache)

- [`cache_dir`](cache) - Returns the directory path where the SQLite database file is stored.
- [`content_hash`](cache) - Computes a SHA-256 hash of a string to generate deterministic, collision-resistant cache keys.
- [`default_db_path`](cache) - Constructs the full file path to the SQLite database within the cache directory.
- [`now_secs`](cache) - Retrieves the current Unix timestamp in seconds for TTL calculations.

### [cli](cli)

- [`cmd_cache_clear`](cli) - Removes cached LLM responses and intermediate artifacts to force fresh analysis.
- [`cmd_chat`](cli) - Enters an interactive REPL loop that queries the LLM using formatted project context and chat history.
- [`cmd_config`](cli) - Manages persistent configuration actions such as setting API keys or selecting default models.
- [`cmd_index`](cli) - Triggers RAG index construction or rebuild for a given codebase path.
- [`cmd_map`](cli) - Generates and displays a dependency graph for a target codebase in the specified format.
- [`cmd_scan`](cli) - Scans a directory tree for code files and outputs a structured scan report.
- [`cmd_serve`](cli) - Initializes and starts the HTTP server, binding to a port and serving the web interface with shared application state.
- [`is_url`](cli) - Determines whether a provided string is a remote Git URL or a local filesystem path.
- [`main`](cli) - Application bootstrap: parses arguments, initializes tokio runtime, resolves configuration, and dispatches to the appropriate command handler.
- [`run_analysis`](cli) - Orchestrates the full documentation pipeline: loads project context, builds RAG index, invokes the analyzer with LLM prompts, and constructs the final wiki.

### [core](core)

- [`resolve_model`](core) - Maps shorthand alias strings to full provider/model identifiers based on a static lookup table.

### [export](export)

- [`export_html`](export) - Orchestrates the full HTML export pipeline, writing rendered pages to the specified output directory.
- [`export_json`](export) - Serializes the Wiki model into JSON format and writes it to the target path.
- [`export_markdown`](export) - Main entry point for Markdown export. Iterates through pages, applies incremental checks, and writes only modified files.
- [`html_escape`](export) - Sanitizes user-generated or LLM-produced text to prevent XSS and malformed HTML.
- [`inline_md`](export) - Processes inline Markdown snippets within larger documents, maintaining link context and formatting rules.
- [`load_state`](export) - Reads the previous export state file from disk to determine which pages require regeneration.
- [`markdown_to_html`](export) - Parses raw Markdown strings into HTML, resolving internal page links and handling formatting directives.
- [`normalize_path`](export) - Standardizes file paths and link references to ensure consistent cross-page navigation.
- [`readme_text`](export) - Constructs the project README Markdown, integrating overview data and navigation links.
- [`save_state`](export) - Writes updated state and page hashes back to disk after a successful export cycle.
- [`serialize_sidebar`](export) - Recursively flattens the sidebar item tree into a JSON-compatible vector structure.
- [`sidebar_text`](export) - Generates Markdown-formatted navigation trees based on the wiki hierarchy.
- [`strip_md_from_internal_links`](export) - Removes Markdown extension suffixes from internal links to ensure cross-platform compatibility.
- [`write_if_changed`](export) - Compares new content against existing files and only performs disk I/O if the content differs.
- [`write_site_loader`](export) - Writes the INDEX_HTML constant to the output directory, configuring the title and redirect behavior.

### [frontend](frontend)

- [`escapeHtml`](frontend) - Sanitizes plain text to prevent XSS when rendering untrusted wiki content.
- [`getFileContent`](frontend) - Fetches raw source file contents linked from the wiki.
- [`getHeaders`](frontend) - Constructs authentication and content-type headers for API requests.
- [`getPage`](frontend) - Retrieves the raw markdown content for a specific wiki page.
- [`getWiki`](frontend) - Fetches the complete wiki structure and metadata for a project.
- [`handleScan`](frontend) - Initiates the backend codebase scan process and updates progress state.
- [`handleSend`](frontend) - Processes user input and triggers the streaming chat endpoint.
- [`markdownToHtml`](frontend) - Converts processed markdown segments into sanitized HTML.
- [`scanProject`](frontend) - Triggers the initial codebase analysis job on the backend.
- [`splitMermaid`](frontend) - Extracts Mermaid code blocks from raw markdown strings.
- [`streamChat`](frontend) - Sends user queries to the LLM and streams incremental responses.
- [`streamScanProgress`](frontend) - Establishes a WebSocket connection to receive real-time scan logs.

### [graph](graph)

- [`get_module`](graph) - Extracts the logical module name from a file path by stripping extensions and directory separators.
- [`import_patterns`](graph) - Returns a vector of compiled Regex patterns tailored to specific programming languages for extracting import statements.
- [`mermaid_id`](graph) - Sanitizes file or module names to produce valid, URL-safe identifiers required by Mermaid.js node definitions.
- [`normalize_path`](graph) - Standardizes file paths by stripping prefixes, resolving symlinks, and ensuring consistent casing for reliable graph node indexing.
- [`pagerank_power_iteration`](graph) - Implements the iterative power method algorithm to converge on stable PageRank scores for each node in the graph.
- [`resolve_import`](graph) - Resolves matched import tokens against the project filesystem to produce normalized, absolute file paths.
- [`resolve_python_module`](graph) - Specialized resolver for Python import syntax, handling relative dots, __init__.py mappings, and package prefixes.

### [index](index)

- [`build_candidates`](index) - Generates potential file paths for an import based on source location and language rules.
- [`build_index`](index) - Public entry point that processes a file list and returns a complete ProjectIndex.
- [`build_outgoing`](index) - Constructs outgoing call edges for a specific symbol based on detected invocation patterns.
- [`build_symbol_index`](index) - Creates a fast lookup map from symbol names to their defining file paths.
- [`compute_metrics`](index) - Calculates complexity and structural metrics for a given code snippet.
- [`extract_go`](index) - Parses Go source to return indexed symbols and imports.
- [`extract_java`](index) - Parses Java source to return indexed symbols and imports.
- [`extract_python`](index) - Parses Python source to return indexed symbols and imports.
- [`extract_rust`](index) - Parses Rust source to return indexed symbols and imports.
- [`extract_symbols`](index) - Dispatches to language-specific extractors based on file extension.
- [`extract_typescript`](index) - Parses TypeScript source to return indexed symbols and imports.
- [`format_module_context`](index) - Generates a concise textual summary of a module's contents for prompt injection.
- [`group_into_module_indices`](index) - Clusters related files into logical ModuleIndex units based on naming and path conventions.
- [`index_one_file`](index) - Handles single-file processing including cache validation and language dispatch.
- [`resolve_imports`](index) - Maps all extracted imports across the project to their actual source locations.
- [`resolve_python_module`](index) - Applies Python sys.path and relative import logic to locate target modules.
- [`trace_flow`](index) - Analyzes extracted symbols to generate CallEdge objects mapping caller-callee relationships.

### [ingest](ingest)

- [`authenticated_clone_url`](ingest) - Constructs a clone-ready URL by injecting resolved authentication tokens when required.
- [`clone_dir`](ingest) - Generates a secure, unique temporary directory path for storing cloned repositories.
- [`dir_size_mb`](ingest) - Calculates the total disk footprint of a directory to enforce the MAX_REPO_SIZE_MB constraint.
- [`guess_project_name`](ingest) - Infers a human-readable project identifier from the root directory name or primary file structure.
- [`ingest_github`](ingest) - Orchestrates the full remote ingestion workflow: parses the URL, resolves credentials, clones the repository, validates its size against MAX_REPO_SIZE_MB, and returns a ProjectContext.
- [`ingest_local`](ingest) - Scans a local directory path, constructs a file tree, generates a ScanReport, and returns a normalized ProjectContext.
- [`parse_git_url`](ingest) - Extracts organization, repository name, and base URL from a standard GitHub HTTP/SSH string using regex.
- [`remove_dir_all`](ingest) - Safely deletes the temporary clone directory after ingestion completes or fails.
- [`resolve_token`](ingest) - Retrieves the GitHub access token from environment variables or configuration to enable private repository access.

### [llm](llm)

- [`build_architecture_prompt`](llm) - Generates prompts targeting cross-module relationships and system design patterns.
- [`build_chat_prompt`](llm) - Formats conversational history and current queries for interactive documentation assistance.
- [`build_module_prompt`](llm) - Constructs focused prompts for analyzing individual modules or directories.
- [`build_overview_prompt`](llm) - Assembles initial context messages using file tree, key files, and language to generate high-level documentation.
- [`extract_json`](llm) - Sanitizes raw LLM text output using regex to isolate and parse valid JSON payloads.
- [`resolve_api_base`](llm) - Normalizes or defaults the API endpoint URL based on the provided model and explicit override.

### [rag](rag)

- [`cosine_similarity`](rag) - Computes the dot product of two normalized vectors divided by their magnitudes, measuring angular similarity between query and chunk vectors.
- [`default_index_dir`](rag) - Returns the filesystem path where the RAG index cache is stored, abstracting OS-specific directory resolution.
- [`format_context`](rag) - Transforms retrieved Chunk objects into a delimited string template optimized for LLM prompt injection and readability.
- [`index_fingerprint`](rag) - Generates a deterministic SHA-256 hash of the project state to detect modifications and invalidate stale cached indices.
- [`load_or_build_index`](rag) - Orchestrates the caching strategy by checking the project fingerprint, loading a cached index if valid, or triggering a full rebuild otherwise.
- [`split_into_chunks`](rag) - Splits source file contents into overlapping or fixed-size blocks at line boundaries to preserve syntactic coherence.
- [`tokenize`](rag) - Normalizes input text into lowercase alphanumeric tokens, stripping punctuation and whitespace for consistent vector mapping.

### [scanner](scanner)

- [`build_file_tree`](scanner) - Concatenates the contents of filtered FileInfo objects into a single delimited string for LLM context injection.
- [`detect_language`](scanner) - Resolves the programming language of a file by checking its extension against lang_map.
- [`from_root`](scanner) - Constructs an IgnoreRules instance by loading standard exclusion patterns relative to the repository root.
- [`glob_match`](scanner) - Lightweight custom glob matcher that compares a pattern against a file path without external dependencies.
- [`has_skipped_suffix`](scanner) - Checks if a path ends with an extension defined in SKIP_EXTS.
- [`is_binary`](scanner) - Inspects raw file bytes to detect binary formats and prevent them from being parsed as text.
- [`is_entrypoint`](scanner) - Determines if a file matches known root configuration or application startup filenames.
- [`is_sensitive_name`](scanner) - Verifies if a filename matches any pattern in SENSITIVE_NAMES.
- [`lang_map`](scanner) - Returns a static lookup table mapping file extensions to canonical language identifiers.
- [`looks_minified_source`](scanner) - Heuristic check to identify compressed JavaScript/TypeScript files that lack readable structure.
- [`matches`](scanner) - Evaluates whether a given relative path and directory flag should be skipped based on loaded patterns.
- [`scan_directory`](scanner) - Main traversal function that uses WalkDir, applies IgnoreRules, classifies files, and returns a ScanReport.
- [`sort_key`](scanner) - Generates a sort tuple that prioritizes entry points and sorts remaining files alphabetically for deterministic output.

### [server](server)

- [`chat`](server) - Processes user questions by retrieving relevant context from the RAG index, constructing prompts, and streaming LLM responses via SSE.
- [`create_app`](server) - Assembles the top-level HTTP router, applies CORS, mounts sub-routers, and injects AppState.
- [`get_file`](server) - Returns raw source file references or snippets linked to documentation entries.
- [`get_graph`](server) - Fetches the serialized dependency graph for frontend visualization.
- [`get_page`](server) - Fetches a specific documentation page by identifier.
- [`get_wiki`](server) - Retrieves the high-level wiki structure and table of contents.
- [`routes`](server) - Mounts the chat handler under the /chat route prefix.
- [`run_scan`](server) - Core workflow engine that resolves configuration, spawns background tasks, and manages lifecycle hooks.
- [`run_scan_inner`](server) - Executes the sequential pipeline: code ingestion, dependency graph construction, LLM analysis, and wiki persistence.
- [`serialize_sidebar`](server) - Converts internal wiki navigation nodes into JSON-compatible structures for UI rendering.
- [`start_scan`](server) - Triggers asynchronous background processing for a new or existing project based on ScanRequest parameters.
- [`stream_status`](server) - Returns real-time scan progress updates via SSE to avoid client polling.

## Interface

### [frontend](frontend)

- [`ChatMessage`](frontend) - Represents a single turn in the AI conversation history.
- [`ChatReference`](frontend) - Defines metadata linking a chat message to a specific wiki page or code artifact.
- [`ContentPart`](frontend) - Represents a parsed segment of markdown, distinguishing between text and diagram blocks.
- [`Props`](frontend) - Defines expected attributes for the diagram wrapper component.
- [`WikiStore`](frontend) - Defines the shape of the global Zustand store including scan status, wiki data, and chat state.

## Method

### [analyzer](analyzer)

- [`analyze`](analyzer) - Top-level entry point that sequentially executes overview, module, architecture, and reading guide generation phases.
- [`analyze_modules`](analyzer) - Iterates through the project index, builds contextual summaries, and spawns concurrent tasks to analyze each module.
- [`generate_architecture`](analyzer) - Analyzes the dependency graph to produce structural documentation about component relationships.
- [`generate_overview`](analyzer) - Constructs and sends a prompt to generate a high-level project summary using the LLM.
- [`generate_reading_guide`](analyzer) - Generates a prioritized sequence of files/modules recommended for new developers to understand the codebase.
- [`new`](analyzer) - Constructor that initializes the analyzer with injected dependencies and sets concurrency limits.

### [cache](cache)

- [`clear`](cache) - Deletes all cached entries and resets the database, returning the count of removed records.
- [`get`](cache) - Retrieves a JSON value by key, returning None if the entry is missing or has exceeded its TTL.
- [`get_default_ttl`](cache) - Fetches a cached value using the DEFAULT_TTL configuration instead of a custom expiration window.
- [`load_project`](cache) - Retrieves previously saved project documentation, returning None if not found or expired.
- [`open`](cache) - Initializes the SQLite database, creates necessary tables if missing, and returns a connected Cache instance.
- [`put`](cache) - Stores a JSON value against a key with an explicit TTL, overwriting existing entries.
- [`save_project`](cache) - Persists project-specific documentation data under a unique project identifier.

### [core](core)

- [`load`](core) - Deserializes configuration from disk into a Config instance, applying defaults for missing fields.
- [`save`](core) - Serializes current configuration state back to disk, enabling persistent user preferences.

### [graph](graph)

- [`build_from_project`](graph) - Parses all files in the ProjectContext, extracts imports using language-specific regex patterns, resolves paths, and populates the directed dependency graph.
- [`edges`](graph) - Returns a list of tuples representing directed import relationships between source and target files.
- [`find_circular_dependencies`](graph) - Applies Tarjan's SCC algorithm to detect cyclic import chains, capped at a specified limit to prevent performance degradation.
- [`find_isolated_files`](graph) - Identifies files with no incoming or outgoing dependency edges, flagging them as potentially undocumented or standalone.
- [`get_core_files`](graph) - Filters the ranked files and returns the top N most important modules based on PageRank thresholds.
- [`get_entry_points`](graph) - Returns files with zero incoming edges, identifying root modules or application entrypoints.
- [`get_module_dependencies`](graph) - Maps each module name to a set of its direct downstream dependencies for quick lookup during documentation generation.
- [`nodes`](graph) - Returns a flat list of all file paths represented as graph nodes.
- [`rank_files`](graph) - Computes and returns a sorted list of files with their PageRank scores to quantify module importance and centrality.
- [`to_mermaid`](graph) - Transforms internal graph nodes and edges into Mermaid.js flowchart syntax for static wiki visualization.

### [llm](llm)

- [`complete`](llm) - Sends a Vec<ChatMessage> to the API and deserializes the full response into a ChatResponse struct.
- [`stream`](llm) - Initiates a streaming request, yielding incremental StreamChunk events as the LLM generates tokens.

### [rag](rag)

- [`index`](rag) - Scans a ProjectContext, splits files into chunks, computes TF-IDF representations, and populates the internal vectors.
- [`load_index`](rag) - Deserializes a previously saved index from disk, returning None if the file is missing or corrupted.
- [`new`](rag) - Initializes an empty SimpleRAG instance with zeroed-out vectors and an empty chunk collection.
- [`retrieve`](rag) - Converts a query string into a TF-IDF vector, calculates cosine similarity against stored chunks, and returns the top-k most relevant results.
- [`save_index`](rag) - Serializes the current index state to a specified file path using serde, enabling persistence across application restarts.

## Struct

### [analyzer](analyzer)

- [`Analyzer`](analyzer) - Main orchestrator holding LLM client, cache, language config, concurrency semaphore, and cache key registry.

### [cache](cache)

- [`Cache`](cache) - Wraps a thread-safe SQLite connection and exposes methods for storing, retrieving, and managing cached data.

### [cli](cli)

- [`Cli`](cli) - Root clap structure defining the top-level CLI schema and routing to subcommands.

### [core](core)

- [`ArchitectureDiagram`](core) - Structured representation of system design including components, sequence flows, and Mermaid diagram definitions.
- [`Config`](core) - Runtime settings container holding API keys, base URLs, model selection, concurrency limits, and file processing thresholds.
- [`ConfigFile`](core) - Disk-persisted format for configuration, containing only essential fields like model, api_key, api_base, and language.
- [`IndexedFile`](core) - Parsed representation of a single source file containing metrics, symbols, imports, exports, and content hash.
- [`IndexedSymbol`](core) - Detailed breakdown of individual code elements including visibility, parameters, return types, decorators, and call targets.
- [`ProjectContext`](core) - Holds repository root, name, and aggregated file tree used to drive LLM context windows.
- [`ProjectIndex`](core) - Aggregated view containing all modules, call graph edges, and global symbol index for cross-file analysis.
- [`ReadingGuide`](core) - Curated step-by-step onboarding plan with time estimates and file references for new developers.
- [`ScanReport`](core) - Categorizes discovered files by size, type, priority, and filtering status during initial traversal.
- [`WikiData`](core) - Final assembled documentation payload composing overview, modules, architecture, and reading guide into a unified output.

### [export](export)

- [`ExportSummary`](export) - Tracks metrics for the export run, including pages written, kept, and removed.
- [`JsonExport`](export) - Root serialization container holding project metadata, page list, and sidebar tree.
- [`PageEntry`](export) - Represents an individual wiki page with its ID, title, content, parent relationship, and sort order.
- [`PageState`](export) - Stores hash signatures of source inputs for a specific page to detect changes.
- [`SidebarEntry`](export) - Models hierarchical navigation items with recursive child support.
- [`StateFile`](export) - Persists export metadata and per-page input hashes to enable incremental updates.

### [graph](graph)

- [`DependencyGraph`](graph) - Holds the underlying petgraph DiGraph and exposes public APIs for graph analysis, ranking, and visualization export.

### [llm](llm)

- [`ChatMessage`](llm) - Standardized data contract representing role/content pairs for system, user, and assistant messages.
- [`LLMClient`](llm) - Core wrapper that holds reqwest client state, model configuration, API credentials, and running totals for tokens and cost.

### [rag](rag)

- [`Chunk`](rag) - Represents a contiguous block of source code with file location metadata and a relevance score calculated during retrieval.
- [`ChunkData`](rag) - Lightweight serialization struct containing only file path and line range metadata, excluding transient fields like scores during persistence.
- [`IndexPayload`](rag) - Internal serialization wrapper that groups chunks, IDF weights, and TF vectors into a single JSON-serializable container.
- [`SimpleRAG`](rag) - Central stateful engine that holds indexed chunks, IDF weights, and term-frequency vectors for similarity matching.

### [scanner](scanner)

- [`IgnoreRules`](scanner) - Holds a collection of glob-style patterns used to filter out unwanted paths.

### [server](server)

- [`AppState`](server) - Global concurrency-safe container holding the document cache and a HashMap of active ProjectState instances.
- [`ChatRequest`](server) - Payload for conversational AI queries containing the question and conversation history.
- [`FileReference`](server) - Structured representation of code snippets linked to documentation, including line ranges and raw text.
- [`ProjectInfo`](server) - Status and metadata returned after scanning completes, including file counts and error states.
- [`ProjectState`](server) - Per-project runtime context tracking metadata, generated wiki, RAG index, and scan progress.
- [`ScanRequest`](server) - Parameters for initiating a documentation scan including path, URL, language, model selection, and API keys.

## Type

### [analyzer](analyzer)

- [`ProgressFn`](analyzer) - Callback type signature for reporting analysis progress to callers.
