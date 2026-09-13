# cache

> Provides persistent, TTL-based key-value storage for LLM-generated documentation and project data.

The cache module abstracts SQLite persistence to store and retrieve LLM outputs efficiently. It prevents redundant API calls by hashing content into deterministic keys, enforces time-to-live (TTL) expiration to keep data fresh, and isolates project-specific caches. This reduces latency and cost while ensuring generated wiki content survives application restarts without requiring external infrastructure.

## Files

### `crates/cache/Cargo.toml`

Declares runtime dependencies for SQLite access, cryptographic hashing, error handling, and JSON serialization required by the cache implementation.

### `crates/cache/src/lib.rs`

Implements the core caching logic, including database initialization, TTL enforcement, content hashing, and project-scoped storage.

- `CacheError` (enum) - Defines errors for database connection failures, query execution issues, and I/O problems.
- `DEFAULT_TTL` (const) - Sets the default expiration duration in seconds for cached entries.
- `cache_dir` (function) - Returns the directory path where the SQLite database file is stored.
- `default_db_path` (function) - Constructs the full file path to the SQLite database within the cache directory.
- `content_hash` (function) - Computes a SHA-256 hash of a string to generate deterministic, collision-resistant cache keys.
- `now_secs` (function) - Retrieves the current Unix timestamp in seconds for TTL calculations.
- `Cache` (struct) - Wraps a thread-safe SQLite connection and exposes methods for storing, retrieving, and managing cached data.
- `open` (method) - Initializes the SQLite database, creates necessary tables if missing, and returns a connected Cache instance.
- `get` (method) - Retrieves a JSON value by key, returning None if the entry is missing or has exceeded its TTL.
- `get_default_ttl` (method) - Fetches a cached value using the DEFAULT_TTL configuration instead of a custom expiration window.
- `put` (method) - Stores a JSON value against a key with an explicit TTL, overwriting existing entries.
- `save_project` (method) - Persists project-specific documentation data under a unique project identifier.
- `load_project` (method) - Retrieves previously saved project documentation, returning None if not found or expired.
- `clear` (method) - Deletes all cached entries and resets the database, returning the count of removed records.

## Key Concepts

- **Content-Addressable Caching**: Uses SHA-256 hashing of raw content to derive cache keys, ensuring identical inputs always map to the same key and preventing duplicate storage.
- **TTL Expiration**: Enforces time-to-live constraints on every read/write operation, automatically invalidating stale LLM outputs to balance freshness with performance.
- **SQLite Persistence**: Leverages a lightweight, single-file relational database for reliable ACID transactions, concurrent access safety, and zero-config deployment.

## Internal Relationships

- `crates/cache/Cargo.toml` → `crates/cache/src/lib.rs`: Cargo.toml supplies the build-time and runtime dependencies (rusqlite, sha2, thiserror, serde_json) that lib.rs directly uses to implement the caching layer.
- `crates/cache/src/lib.rs` → `main application`: Consumed by the CLI and web server modules to intercept LLM requests, cache responses, and serve data without blocking on network calls.
