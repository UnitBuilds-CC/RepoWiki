# server

> Exposes an HTTP API to orchestrate codebase scanning, LLM-powered documentation generation, and interactive wiki retrieval.

The server module serves as the web-facing control plane for the Repowiki system. It manages concurrent project states, coordinates background scanning workflows, streams real-time progress via Server-Sent Events, and serves the generated documentation and dependency graphs. It decouples heavy computational work (ingestion, analysis, RAG indexing) into dedicated workspace crates while providing a standardized REST/SSE interface for both CLI tools and frontend applications.

## Files

### `crates/server/Cargo.toml`

Defines crate metadata and declares dependencies for Axum, Tokio, Serde, and internal workspace crates required for HTTP serving and async execution.

### `crates/server/src/lib.rs`

Application entry point that initializes shared state and configures the Axum router with CORS middleware and route mounting.

- `AppState` (struct) - Global concurrency-safe container holding the document cache and a HashMap of active ProjectState instances.
- `ProjectState` (struct) - Per-project runtime context tracking metadata, generated wiki, RAG index, and scan progress.
- `create_app` (function) - Assembles the top-level HTTP router, applies CORS, mounts sub-routers, and injects AppState.

### `crates/server/src/models.rs`

Defines HTTP request/response payloads and DTOs for API communication, enabling serialization between clients and handlers.

- `ScanRequest` (struct) - Parameters for initiating a documentation scan including path, URL, language, model selection, and API keys.
- `ProjectInfo` (struct) - Status and metadata returned after scanning completes, including file counts and error states.
- `ChatRequest` (struct) - Payload for conversational AI queries containing the question and conversation history.
- `FileReference` (struct) - Structured representation of code snippets linked to documentation, including line ranges and raw text.

### `crates/server/src/routers/mod.rs`

Declares routing submodules for Axum, organizing endpoint logic into separate files.

### `crates/server/src/routers/chat.rs`

Implements AI-assisted querying endpoints with streaming support for interactive documentation exploration.

- `routes` (function) - Mounts the chat handler under the /chat route prefix.
- `chat` (function) - Processes user questions by retrieving relevant context from the RAG index, constructing prompts, and streaming LLM responses via SSE.

### `crates/server/src/routers/scan.rs`

Orchestrates codebase ingestion and documentation generation through asynchronous background workflows.

- `routes` (function) - Mounts scan management endpoints for starting, querying, and monitoring projects.
- `start_scan` (function) - Triggers asynchronous background processing for a new or existing project based on ScanRequest parameters.
- `stream_status` (function) - Returns real-time scan progress updates via SSE to avoid client polling.
- `run_scan` (function) - Core workflow engine that resolves configuration, spawns background tasks, and manages lifecycle hooks.
- `run_scan_inner` (function) - Executes the sequential pipeline: code ingestion, dependency graph construction, LLM analysis, and wiki persistence.

### `crates/server/src/routers/wiki.rs`

Serves generated documentation assets, navigation structures, and dependency visualizations to clients.

- `routes` (function) - Mounts wiki reading endpoints under the /wiki route prefix.
- `get_wiki` (function) - Retrieves the high-level wiki structure and table of contents.
- `get_page` (function) - Fetches a specific documentation page by identifier.
- `get_file` (function) - Returns raw source file references or snippets linked to documentation entries.
- `get_graph` (function) - Fetches the serialized dependency graph for frontend visualization.
- `serialize_sidebar` (function) - Converts internal wiki navigation nodes into JSON-compatible structures for UI rendering.

## Key Concepts

- **AppState & ProjectState**: Centralized concurrency-safe containers that isolate per-project runtime data, enabling parallel scan operations without race conditions.
- **Server-Sent Events (SSE)**: Streaming protocol used to push real-time progress from long-running scans and token-by-token LLM responses to clients without requiring polling.
- **RAG Index Integration**: The chat router dynamically loads or rebuilds vector indexes from scanned codebases to ground LLM answers in actual repository context rather than generic training data.
- **Async Workflow Orchestration**: Scanning is offloaded to background tasks; the API remains responsive while ingestion, graph construction, and LLM prompting execute sequentially in isolated scopes.

## Internal Relationships

- `lib.rs` → `routers/*.rs`: lib.rs aggregates all router modules into a single Axum Router and shares AppState across them via Arc.
- `routers/scan.rs` → `lib.rs`: Scan handlers read and mutate ProjectState instances stored in AppState to track ongoing analyses and progress.
- `routers/chat.rs` → `routers/scan.rs`: Chat queries depend on successfully completed scans to populate the RAG index and wiki data before answering.
- `models.rs` → `routers/*.rs`: All routers deserialize incoming JSON into models.rs structs and serialize responses back out using Serde.
- `routers/wiki.rs` → `routers/scan.rs`: Wiki endpoints serve data that was previously computed, analyzed, and persisted by the scan workflow.
