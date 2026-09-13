# cache

> Provides persistent, TTL-managed caching for structured data and project documents to reduce redundant LLM processing.

This module implements an embedded SQLite-backed cache that stores JSON-serializable values with optional time-to-live (TTL) expiration. It supports the project's structured indexing pipeline by deduplicating identical inputs via SHA-256 hashing and persisting analysis results locally. This minimizes API calls and latency when regenerating or querying wiki documentation. The module exposes both generic key-value operations and project-specific document storage, along with utilities for path resolution and cache invalidation.

## Files

### `crates/cache/Cargo.toml`

Defines package metadata and declares runtime dependencies for the cache module, including SQLite bindings, JSON serialization, cryptographic hashing, and error handling.

### `crates/cache/src/lib.rs`

Implements the core caching logic using rusqlite, providing TTL-aware storage, content hashing, and project-scoped document management.

- `CacheError` (enum) - Defines error types for database operations, IO failures, and cache mismatches.
- `DEFAULT_TTL` (const) - Sets the default expiration duration for cached entries in seconds.
- `cache_dir` (function) - Returns the standard directory path where the cache database and related files are stored.
- `default_db_path` (function) - Constructs the full file path to the SQLite database within the cache directory.
- `content_hash` (function) - Computes a SHA-256 hex digest of a string to enable deterministic deduplication of identical code/content.
- `now_secs` (function) - Returns the current Unix timestamp as a float for TTL calculations.
- `Cache` (struct) - Wraps a rusqlite Connection and provides methods for reading, writing, and managing cached data.
- `Cache::open` (method) - Initializes or opens the SQLite database at the specified path, creating tables if necessary.
- `Cache::get` (method) - Retrieves a cached JSON value by key, returning None if expired or missing.
- `Cache::get_default_ttl` (method) - Fetches a cached value using DEFAULT_TTL for expiration checks.
- `Cache::put` (method) - Stores a JSON value under a key with a custom TTL, overwriting existing entries.
- `Cache::save_project` (method) - Persists a complete project document or index state under a project-scoped key.
- `Cache::load_project` (method) - Loads a previously saved project document or index state by ID.
- `Cache::clear` (method) - Drops all cached entries and resets the database, returning the number of removed rows.

## Key Concepts

- **TTL Expiration**: Cached entries automatically expire after a configurable duration, ensuring stale indexes are refreshed without manual intervention.
- **Content Hashing**: SHA-256 fingerprints allow the system to detect identical inputs across runs, enabling cache hits even when logical keys differ.
- **SQLite Persistence**: Embedded relational storage provides ACID guarantees and efficient range queries for large-scale wiki indexing without external services.
- **Project-Scoped Caching**: Separates per-project document states from generic caches, allowing independent lifecycle management and incremental updates.

## Internal Relationships

- `crates/cache/Cargo.toml` → `crates/cache/src/lib.rs`: Cargo.toml declares the dependencies (rusqlite, serde_json, sha2, thiserror) required by lib.rs to implement the SQLite-backed cache.
- `crates/cache/src/lib.rs` → `other crates (indexer, llm_client, cli)`: Consumed by the indexing and generation pipelines to store/retrieve parsed code structures and LLM outputs, preventing redundant processing.
