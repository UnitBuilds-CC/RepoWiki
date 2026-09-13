# Reading Guide

This codebase separates a Rust-based backend engine from a TypeScript frontend. Navigate it by starting with configuration and entry points to grasp the build/runtime setup, then trace the data flow through the core indexing and LLM orchestration modules, and finally examine the frontend views and export pipeline. Focus on how cost-optimization strategies (caching, graph prioritization, RAG) thread through the entire stack and how state flows between the global store and UI components.

## Step 1: Configuration & Project Scaffold (~5 min)

**Files:** [`.env.example`](root), [`Cargo.toml`](root), [`README.md`](root), [`crates/analyzer/Cargo.toml`](analyzer), [`crates/core/Cargo.toml`](core)

Identify required environment variables, workspace organization, and dependency versions. Look for how the Rust crates are partitioned, feature flags, build profiles, and how the CLI versus server binaries are configured to understand the project's operational boundaries and startup requirements.

## Step 2: Frontend Entry Point & Global State (~5 min)

**Files:** [`frontend/src/App.tsx`](frontend), [`frontend/src/stores/wiki.ts`](frontend)

Trace how the application initializes, sets up routing, and mounts root components. Examine the global store structure to understand how wiki data, chat history, and UI state are persisted, synchronized, and consumed across different views before diving into individual pages.

## Step 3: API Abstraction & Client Contract (~5 min)

**Files:** [`frontend/src/lib/api.ts`](frontend)

Analyze how the frontend abstracts HTTP requests, handles headers/authentication, manages error responses, and maps to backend endpoints. This file defines the communication contract between the UI and the Rust server, making it critical for debugging integration issues and understanding request lifecycle.

## Step 4: Core Indexing & Source Ingestion Pipeline (~15 min)

**Files:** `crates/core/`, `crates/index/`, `crates/scanner/`, `crates/ingest/`

Follow the data flow from raw repository cloning or local directory scanning to structured metadata extraction. Look for intelligent filtering rules, language detection, AST/parsing strategies, and how source files are normalized before being passed downstream for analysis.

## Step 5: LLM Orchestration & Cost Optimization (~20 min)

**Files:** `crates/llm/`, `crates/analyzer/`, `crates/cache/`, `crates/graph/`, `crates/rag/`

Study how prompts are constructed, how external LLM APIs are abstracted, and how token consumption is minimized. Pay close attention to the caching TTL strategy, graph-based prioritization for documentation targets, and the deterministic retrieval pipeline that feeds only relevant snippets to the model.

## Step 6: Server API & CLI Interface (~10 min)

**Files:** `crates/server/`, `crates/cli/`

Map out HTTP endpoints, request/response schemas, and streaming progress mechanisms for the web interface. For the CLI, identify command structures, argument parsing, and how users trigger scans or interact with the chat assistant locally.

## Step 7: Frontend Views & Interactive Components (~15 min)

**Files:** [`frontend/src/pages/Home.tsx`](frontend), [`frontend/src/pages/WikiView.tsx`](frontend), [`frontend/src/pages/ChatView.tsx`](frontend), [`frontend/src/components/WikiContent.tsx`](frontend), [`frontend/src/components/WikiSidebar.tsx`](frontend), [`frontend/src/components/MermaidDiagram.tsx`](frontend), [`frontend/src/components/SettingsModal.tsx`](frontend)

Observe how pages compose shared components, handle user interactions like chat queries and wiki navigation, render Mermaid diagrams, and manage modal states. Note how state from the global store flows down to these presentational components and how side effects are triggered.

## Step 8: Static Export & Documentation Distribution (~10 min)

**Files:** `crates/export/`

Review how internally analyzed wiki structures are transformed into static output formats. Look for template engines, asset bundling logic, and how the final documentation package is assembled and optimized for distribution or hosting.

## Tips

- Use your IDE's Find Usages or Go to Definition to trace cross-crate dependencies, especially around the strongly-typed data models in the core module.
- Check the .github/workflows directory early to understand CI/CD pipelines, testing strategies, and deployment gates before making changes.
- The frontend heavily relies on api.ts for all backend communication; consider mocking its responses to isolate and test UI components independently.
- Token cost optimization is a first-class architectural concern; track how cache, graph, and rag modules collaborate to reduce redundant LLM processing during development.
- Run the CLI in dry-run or verbose mode when experimenting with the scanner and indexer to observe file classification and context window boundaries without triggering full LLM calls.
