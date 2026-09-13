# ingest

> Acquires and normalizes source code from GitHub repositories and local directories into a standardized format for downstream structured indexing.

The ingest module serves as the acquisition layer for the documentation pipeline, responsible for fetching source code and extracting structural metadata before it reaches the LLM. It provides two distinct ingestion strategies: cloning remote GitHub repositories and scanning local filesystem directories. By enforcing strict size limits, handling authentication securely, and returning a unified representation of the codebase, it filters out invalid or oversized inputs early. This pre-processing step is critical for controlling token consumption, ensuring consistent data shapes for the indexer, and preventing resource exhaustion during documentation generation.

## Files

### `crates/ingest/Cargo.toml`

Defines package metadata, versioning, and runtime dependencies for the ingest crate.

### `crates/ingest/src/github.rs`

Manages GitHub-specific repository acquisition, including URL parsing, authentication, safe cloning, size validation, and cleanup.

- `MAX_REPO_SIZE_MB` (constant) - Hard limit on repository size in megabytes to prevent resource exhaustion and control LLM costs.
- `clone_dir` (function) - Creates an isolated temporary directory for safely cloning repositories without polluting the host filesystem.
- `git_url_regex` (constant) - Compiled regex pattern used to validate and extract owner, repository, and branch components from Git URLs.
- `parse_git_url` (function) - Extracts and validates the owner, repository name, and branch/tag from a raw Git URL string.
- `clone_url` (function) - Constructs the base git clone command string for unauthenticated access.
- `resolve_token` (function) - Securely retrieves a GitHub personal access token from environment variables for private repository access.
- `authenticated_clone_url` (function) - Injects resolved credentials into the clone URL to enable secure access to restricted repositories.
- `remove_dir_all` (function) - Safely deletes the temporary clone directory and all its contents after processing completes.
- `dir_size_mb` (function) - Recursively calculates the total size of a cloned directory in megabytes for size validation.
- `ingest_github` (function) - Main orchestrator that clones, validates size, extracts metadata, and returns a standardized ProjectContext and ScanReport.

### `crates/ingest/src/lib.rs`

Crate root that re-exports public ingestion functions and organizes module visibility.

### `crates/ingest/src/local.rs`

Handles local filesystem ingestion by scanning directories, building file trees, and extracting standardized metadata.

- `guess_project_name` (function) - Derives a readable project identifier from the root directory path or contained file names.
- `ingest_local` (function) - Orchestrates local directory scanning, delegates to repowiki_scanner, and returns a unified ProjectContext and ScanReport.

## Key Concepts

- **Parallel Ingestion Paths**: GitHub cloning and local scanning are implemented separately but normalized to identical output types, enabling a single downstream indexing pipeline to handle both seamlessly.
- **Cost Control via Pre-filtering**: Hard size limits and early validation prevent oversized or malformed repositories from consuming expensive LLM tokens during documentation generation.
- **Secure Credential Injection**: Authentication tokens are resolved at runtime from environment variables and only injected into clone URLs when necessary, avoiding hardcoded secrets and supporting private repositories.
- **Temporary Isolation**: Repositories are cloned into ephemeral directories that are explicitly cleaned up after processing, preventing host filesystem pollution and race conditions.

## Internal Relationships

- `crates/ingest/src/lib.rs` → `crates/ingest/src/github.rs`: Re-exports ingest_github to expose it as part of the crate's public API.
- `crates/ingest/src/lib.rs` → `crates/ingest/src/local.rs`: Re-exports ingest_local to expose it as part of the crate's public API.
- `crates/ingest/src/github.rs` → `repowiki_core::models`: Returns standardized ProjectContext and ScanReport structs to ensure downstream modules receive a consistent data shape regardless of source.
- `crates/ingest/src/local.rs` → `repowiki_scanner`: Delegates actual directory traversal, file enumeration, and metadata extraction to the dedicated scanner crate.
- `crates/ingest/src/github.rs` → `crates/ingest/src/local.rs`: Both implement parallel ingestion paths that converge on identical output structures, allowing the indexer to treat remote and local sources uniformly.
