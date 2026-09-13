# rag

> Provides a deterministic, cost-optimized retrieval pipeline that indexes codebases and fetches relevant snippets to minimize LLM token consumption.

The rag module implements a lightweight Retrieval-Augmented Generation system tailored for source code analysis. It replaces expensive neural embeddings and external vector databases with a statistical TF-IDF ranking engine paired with cosine similarity. The module manages the full indexing lifecycle: parsing a ProjectContext into fixed-size line chunks, computing term frequency-inverse document frequency weights, persisting the index to disk, and efficiently retrieving the top-k most relevant code slices for natural language queries. A SHA-256 fingerprinting mechanism detects codebase changes, triggering index rebuilds only when necessary to avoid redundant computation. Retrieved chunks are serialized into compact, prompt-optimized strings that maximize LLM comprehension per token, directly supporting the project's architectural goal of drastically reducing inference costs while maintaining high contextual precision.

## Files

### `crates/rag/Cargo.toml`

Declares minimal runtime dependencies including serde for serialization, sha2 for change detection, regex for tokenization, and repowiki_core for project model access. Explicitly excludes heavy ML or vector database crates to maintain a lean binary footprint.

### `crates/rag/src/lib.rs`

Contains the complete RAG implementation. Exports the SimpleRAG orchestrator, Chunk data structure, and utility functions for tokenization, similarity calculation, chunk splitting, and JSON persistence. Implements the deterministic ranking and retrieval workflow.

- `SimpleRAG` (struct) - Core orchestrator storing indexed chunks, IDF weights, and TF vectors. Manages indexing, persistence, and query execution.
- `index` (method) - Processes a ProjectContext, splits source files into bounded line chunks, computes TF-IDF statistics, and populates internal vectors.
- `retrieve` (method) - Tokenizes a query, calculates cosine similarity against stored TF vectors, and returns the top-k highest-scoring Chunks.
- `load_or_build_index` (function) - Validates project state against a cached SHA-256 fingerprint. Loads existing index if unchanged, otherwise triggers a full rebuild.
- `format_context` (function) - Converts retrieved Chunk slices into a delimited string optimized for direct injection into LLM system prompts.
- `tokenize` (function) - Splits raw text into alphanumeric tokens using regex, stripping punctuation and normalizing case for consistent vector mapping.
- `cosine_similarity` (function) - Computes dot product of normalized TF vectors to determine semantic alignment between query and indexed chunks.
- `split_into_chunks` (function) - Divides file contents into fixed-length line ranges, attaching file path and line boundaries to each segment.
- `IndexPayload` (struct) - Serde-compatible DTO wrapping chunks, IDF map, and TF vectors for safe JSON serialization and deserialization.
- `index_fingerprint` (function) - Generates a deterministic SHA-256 hash from project metadata to detect structural or content changes.

## Key Concepts

- **TF-IDF Cosine Ranking**: Replaces neural embeddings with statistical word-frequency weighting to compute relevance scores deterministically, eliminating GPU requirements and external service dependencies.
- **Fingerprint-Based Caching**: Uses SHA-256 hashing of project metadata to invalidate caches only when source code actually changes, preventing redundant re-indexing and preserving compute budget.
- **Fixed-Size Line Chunks**: Splits files into bounded line ranges rather than semantic blocks, ensuring predictable memory allocation and enabling precise line-number references for developer navigation.
- **Prompt-Optimized Formatting**: Transforms raw code slices into compact, structured strings specifically designed to maximize LLM comprehension per token while minimizing context window waste.

## Internal Relationships

- `Cargo.toml` → `src/lib.rs`: Cargo.toml declares the serde, sha2, regex, and repowiki_core dependencies required by lib.rs for serialization, hashing, pattern matching, and project model consumption.
- `src/lib.rs` → `repowiki_core::models::ProjectContext`: lib.rs consumes ProjectContext to extract file paths and raw source content during the initial indexing phase.
- `SimpleRAG` → `Chunk`: SimpleRAG aggregates multiple Chunk instances as the fundamental retrievable unit, attaching computed relevance scores during query execution.
- `load_or_build_index` → `SimpleRAG::save_index / SimpleRAG::load_index`: Orchestrates disk persistence by delegating to save/load methods after validating whether the project fingerprint matches the cached index.
