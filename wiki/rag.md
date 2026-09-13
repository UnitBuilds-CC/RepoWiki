# rag

> Provides a lightweight, TF-IDF-based retrieval system that indexes codebases and fetches relevant source snippets for LLM-driven documentation generation.

The rag module implements a custom Retrieval-Augmented Generation pipeline optimized for source code. It parses project files into fixed-size chunks, computes Term Frequency-Inverse Document Frequency (TF-IDF) vectors for each chunk, and uses cosine similarity to rank code snippets against natural language queries. This enables downstream components to inject only highly relevant code contexts into LLM prompts, conserving tokens and improving documentation accuracy. The module manages the full lifecycle including indexing, caching, cache invalidation via project fingerprinting, and serialization.

## Files

### `crates/rag/Cargo.toml`

Defines the Rust crate manifest, declaring dependencies for serialization (serde), cryptographic hashing (sha2), and text processing (regex) required by the core engine.

### `crates/rag/src/lib.rs`

Contains the core RAG engine. Implements chunking, TF-IDF vectorization, cosine similarity search, disk persistence, and prompt formatting. Exposes the public API for indexing projects and retrieving relevant code contexts.

- `INDEX_DIR_NAME` (constant) - Defines the standard directory name used to store cached retrieval indices on disk.
- `Chunk` (struct) - Represents a contiguous block of source code with file location metadata and a relevance score calculated during retrieval.
- `default_index_dir` (function) - Returns the filesystem path where the RAG index cache is stored, abstracting OS-specific directory resolution.
- `index_fingerprint` (function) - Generates a deterministic SHA-256 hash of the project state to detect modifications and invalidate stale cached indices.
- `SimpleRAG` (struct) - Central stateful engine that holds indexed chunks, IDF weights, and term-frequency vectors for similarity matching.
- `new` (method) - Initializes an empty SimpleRAG instance with zeroed-out vectors and an empty chunk collection.
- `index` (method) - Scans a ProjectContext, splits files into chunks, computes TF-IDF representations, and populates the internal vectors.
- `save_index` (method) - Serializes the current index state to a specified file path using serde, enabling persistence across application restarts.
- `load_index` (method) - Deserializes a previously saved index from disk, returning None if the file is missing or corrupted.
- `retrieve` (method) - Converts a query string into a TF-IDF vector, calculates cosine similarity against stored chunks, and returns the top-k most relevant results.
- `load_or_build_index` (function) - Orchestrates the caching strategy by checking the project fingerprint, loading a cached index if valid, or triggering a full rebuild otherwise.
- `format_context` (function) - Transforms retrieved Chunk objects into a delimited string template optimized for LLM prompt injection and readability.
- `tokenize` (function) - Normalizes input text into lowercase alphanumeric tokens, stripping punctuation and whitespace for consistent vector mapping.
- `cosine_similarity` (function) - Computes the dot product of two normalized vectors divided by their magnitudes, measuring angular similarity between query and chunk vectors.
- `split_into_chunks` (function) - Splits source file contents into overlapping or fixed-size blocks at line boundaries to preserve syntactic coherence.
- `IndexPayload` (struct) - Internal serialization wrapper that groups chunks, IDF weights, and TF vectors into a single JSON-serializable container.
- `ChunkData` (struct) - Lightweight serialization struct containing only file path and line range metadata, excluding transient fields like scores during persistence.

## Key Concepts

- **TF-IDF Vectorization**: Converts unstructured code text into numerical vectors where term importance is weighted by global document frequency, enabling efficient similarity searches without heavy ML models or external APIs.
- **Cache Invalidation via Fingerprinting**: Uses SHA-256 hashing of project state to automatically detect when the codebase changes, ensuring the retrieval index stays fresh without manual intervention or full re-scans.
- **Boundary-Aware Chunking**: Splits source files at line boundaries rather than arbitrary character counts, preserving syntactic coherence and making it easier to reference specific code locations in generated documentation.
- **Prompt Context Formatting**: Structures retrieved code snippets with file paths and line ranges into a standardized string format, optimizing how LLMs parse, understand, and utilize injected context.

## Internal Relationships

- `crates/rag/Cargo.toml` → `crates/rag/src/lib.rs`: Manifest declares runtime dependencies required by the core implementation for serialization, hashing, and regex operations.
- `crates/rag/src/lib.rs` → `crates/rag/src/lib.rs`: The indexing pipeline feeds processed data into the persistence layer, which serializes IndexPayload to disk for later retrieval.
- `crates/rag/src/lib.rs` → `crates/rag/src/lib.rs`: Retrieved chunks are passed directly to format_context before being returned to callers for prompt construction.
- `crates/rag/src/lib.rs` → `crates/rag/src/lib.rs`: Fingerprint check gates whether the cached index is loaded or rebuilt from scratch in load_or_build_index.
