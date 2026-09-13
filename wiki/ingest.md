# ingest

> Acquires and normalizes source code from remote Git repositories or local directories into a unified ProjectContext for downstream analysis.

The ingest module abstracts repository acquisition into two parallel strategies: remote cloning and local scanning. It ensures that regardless of the source, the output conforms to a standardized ProjectContext model expected by the core analysis pipeline. Remote ingestion handles Git URL parsing, credential resolution, safe temporary directory management, and size constraints to prevent resource exhaustion. Local ingestion traverses the filesystem, infers project metadata, and delegates structural scanning to the repowiki_scanner crate. By isolating these concerns, the module keeps the core LLM-driven documentation generator agnostic to how the codebase was obtained.

## Files

### `crates/ingest/Cargo.toml`

Defines the Rust crate metadata and dependencies for the ingest module.

### `crates/ingest/src/lib.rs`

Module entry point that aggregates and re-exports the public ingestion interfaces.

### `crates/ingest/src/github.rs`

Manages remote repository acquisition via Git. Handles URL normalization, authentication, safe cloning to temporary storage, size validation, and cleanup.

- `ingest_github` (function) - Orchestrates the full remote ingestion workflow: parses the URL, resolves credentials, clones the repository, validates its size against MAX_REPO_SIZE_MB, and returns a ProjectContext.
- `parse_git_url` (function) - Extracts organization, repository name, and base URL from a standard GitHub HTTP/SSH string using regex.
- `authenticated_clone_url` (function) - Constructs a clone-ready URL by injecting resolved authentication tokens when required.
- `resolve_token` (function) - Retrieves the GitHub access token from environment variables or configuration to enable private repository access.
- `clone_dir` (function) - Generates a secure, unique temporary directory path for storing cloned repositories.
- `remove_dir_all` (function) - Safely deletes the temporary clone directory after ingestion completes or fails.
- `dir_size_mb` (function) - Calculates the total disk footprint of a directory to enforce the MAX_REPO_SIZE_MB constraint.

### `crates/ingest/src/local.rs`

Handles local filesystem traversal and metadata extraction, delegating structural analysis to the scanner crate.

- `ingest_local` (function) - Scans a local directory path, constructs a file tree, generates a ScanReport, and returns a normalized ProjectContext.
- `guess_project_name` (function) - Infers a human-readable project identifier from the root directory name or primary file structure.

## Key Concepts

- **Unified Context Output**: Regardless of source (remote Git or local disk), both ingestion paths converge on a single ProjectContext struct, allowing downstream LLM pipelines to operate without source-specific branching.
- **Safe Temporary Cloning**: Remote ingestion isolates downloaded repositories in ephemeral directories with strict size limits and guaranteed cleanup, preventing disk exhaustion and credential leakage.
- **Separation of Acquisition and Analysis**: Ingest only fetches and structures raw code; it delegates deep file scanning and tree building to repowiki_scanner, keeping the module focused on data acquisition and normalization.

## Internal Relationships

- `crates/ingest/src/lib.rs` → `crates/ingest/src/github.rs`: lib.rs re-exports ingest_github to expose the remote ingestion capability to consumers.
- `crates/ingest/src/lib.rs` → `crates/ingest/src/local.rs`: lib.rs re-exports ingest_local to expose the local ingestion capability to consumers.
- `crates/ingest/src/github.rs` → `crates/ingest/src/local.rs`: github.rs imports ingest_local, likely for shared utility logic or fallback handling during ingestion workflows.
- `crates/ingest/src/local.rs` → `repowiki_scanner`: local.rs delegates file enumeration and tree construction to repowiki_scanner::build_file_tree and scan_directory to produce the ScanReport.
- `crates/ingest/src/github.rs` → `repowiki_core::models`: Both github.rs and local.rs return repowiki_core::models::ProjectContext, ensuring type consistency across ingestion sources.
- `crates/ingest/src/github.rs` → `std::process::Command`: github.rs executes git clone commands externally to fetch remote repositories.
