# server

> Provides an HTTP API layer for initiating codebase scans, streaming progress, querying documentation via chat, and retrieving generated wiki content.

The server module exposes a RESTful and SSE-based web interface that orchestrates the documentation generation pipeline. It manages global and per-project state, handles ingestion requests for local directories or GitHub repositories, streams real-time progress via Server-Sent Events, and serves the resulting wiki structure, pages, and dependency graphs. It also integrates with an LLM client and RAG system to power contextual chat queries against the indexed codebase, reducing token costs by leveraging pre-built structured indexes instead of raw context injection.

## Files

### `crates/server/Cargo.toml`

Package manifest defining dependencies for the web server crate, including Axum, Tokio, and internal workspace crates.

### `crates/server/src/lib.rs`

Core module initialization. Defines AppState and ProjectState structs to manage concurrency-safe registries. Exposes create_app to assemble the Axum router with CORS middleware.

- `AppState` (struct) - Global application state holding the cache and a mutex-protected map of active projects.
- `ProjectState` (struct) - Per-project runtime context storing metadata, wiki instance, RAG index, and progress tracking.
- `create_app` (function) - Constructs and returns the root Axum Router, wiring routes and injecting AppState.

### `crates/server/src/models.rs`

Defines DTOs for API payloads. Standardizes serialization for scan requests, chat interactions, and project status responses.

- `ScanRequest` (struct) - Schema for triggering a new documentation scan with path, URL, language, and model configuration.
- `ProjectInfo` (struct) - Response payload reporting scan status, file counts, line counts, and errors.
- `ChatRequest` (struct) - Schema for chat queries containing the question and conversation history.

### `crates/server/src/routers/mod.rs`

Module declaration bundling scan, chat, and wiki route definitions under a single namespace.

### `crates/server/src/routers/scan.rs`

Endpoint handlers for repository scanning. Triggers ingestion pipelines, initializes dependency graphs, and streams scan progress/status updates via SSE.

- `start_scan` (function) - POST endpoint that validates input, generates a project ID, and spawns a background task to run the scan.
- `stream_status` (function) - GET endpoint that establishes an SSE connection to stream real-time scan progress to the client.
- `run_scan` (function) - Background executor that coordinates ingestion, graph building, and wiki generation, updating AppState upon completion.

### `crates/server/src/routers/chat.rs`

Chat endpoint handler. Processes user questions, retrieves relevant code snippets from the RAG index, constructs optimized prompts, and streams LLM responses via SSE.

- `chat` (function) - POST endpoint that queries the RAG index, builds a prompt using repowiki_llm, and streams the LLM's token-by-token response.

### `crates/server/src/routers/wiki.rs`

Documentation retrieval handlers. Serves the generated wiki sidebar, individual markdown pages, source file snippets, and the project dependency graph as JSON.

- `get_wiki` (function) - GET endpoint returning the top-level wiki structure and sidebar items.
- `get_page` (function) - GET endpoint fetching rendered markdown content for a specific wiki page.
- `get_graph` (function) - GET endpoint returning the serialized dependency graph for visualization.

## Key Concepts

- **Concurrent State Management**: AppState uses Arc<Mutex<HashMap>> to safely handle multiple simultaneous projects and scans without blocking the HTTP server, enabling scalable multi-tenant operation.
- **Server-Sent Events (SSE)**: Used for both scan progress and chat responses to provide real-time, unidirectional streaming feedback to clients without requiring WebSocket overhead or polling.
- **Structured RAG Indexing**: The chat router leverages pre-indexed code dependencies and snippets rather than injecting full codebases into LLM context windows, directly addressing the project's goal of cutting LLM costs.
- **Route Domain Separation**: Scanning, chat, and wiki retrieval are isolated into distinct router modules to maintain clear boundaries of responsibility while sharing a unified AppState for cross-cutting concerns.

## Internal Relationships

- `crates/server/src/lib.rs` → `crates/server/src/models.rs`: lib.rs imports model structs to type-check and deserialize API payloads within route handlers.
- `crates/server/src/routers/scan.rs` → `crates/server/src/lib.rs`: Scan routes read/write AppState and ProjectState to track ongoing scans, store RAG indices, and persist generated wiki data.
- `crates/server/src/routers/chat.rs` → `crates/server/src/lib.rs`: Chat routes depend on AppState to access the SimpleRAG index and ProjectState to validate that a project has been successfully scanned before querying.
- `crates/server/src/routers/scan.rs` → `crates/server/src/routers/wiki.rs`: Scan completion populates the wiki and dependency graph data structures that wiki routes subsequently serve to clients.
- `crates/server/src/routers/chat.rs` → `crates/server/src/routers/scan.rs`: Both routers share the same AppState lifecycle; chat is only functional after scan.rs has finished indexing the codebase.
