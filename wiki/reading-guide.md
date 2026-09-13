# Reading Guide

Approach this codebase by first understanding the setup and entry points, then tracing the frontend state and UI flow, and finally diving into the backend orchestration pipeline and supporting utilities. This layered approach mirrors how data flows from user interaction through LLM processing to final documentation output.

## Step 1: Configuration & Onboarding (~5 min)

**Files:** [`README.md`](root), [`Cargo.toml`](root), [`.env.example`](root)

Identify the project's primary purpose, build system requirements, and dependency tree. Locate environment variables for LLM providers, API keys, and cache paths to understand local setup prerequisites before execution.

## Step 2: Frontend Entry Point & Routing (~5 min)

**Files:** [`frontend/src/App.tsx`](frontend)

Trace how the application mounts, defines routes, and manages global layout. Determine which pages are loaded dynamically, identify top-level context providers, and see how the UI delegates control to specific views.

## Step 3: State Management & API Abstraction (~10 min)

**Files:** [`frontend/src/lib/api.ts`](frontend), [`frontend/src/stores/wiki.ts`](frontend)

Examine how HTTP requests are constructed, authenticated, and handled for errors or retries. Review the state store to see how wiki documents, chat sessions, and scan progress are persisted, updated, and shared across components without prop drilling.

## Step 4: Core UI Views & Interactive Components (~15 min)

**Files:** [`frontend/src/pages/Home.tsx`](frontend), [`frontend/src/pages/ChatView.tsx`](frontend), [`frontend/src/pages/WikiView.tsx`](frontend), [`frontend/src/components/WikiContent.tsx`](frontend), [`frontend/src/components/WikiSidebar.tsx`](frontend), [`frontend/src/components/MermaidDiagram.tsx`](frontend), [`frontend/src/components/SettingsModal.tsx`](frontend)

Follow the user journey from initiating a codebase scan to querying the assistant and browsing generated docs. Look for how dynamic content is rendered, how Mermaid diagrams integrate with the wiki sidebar, and how settings modals override default configurations.

## Step 5: Backend Orchestration & Data Contracts (~10 min)

**Files:** [`crates/cli/Cargo.toml`](cli), `crates/core/Cargo.toml`

Understand how the CLI acts as the unified bridge between terminal usage and the web interface. Inspect the central data contracts and configuration managers that standardize ProjectContext structures across scanning, analysis, and export phases.

## Step 6: Analysis Pipeline & LLM Integration (~20 min)

**Files:** [`crates/analyzer/Cargo.toml`](analyzer), [`crates/index/Cargo.toml`](index)

Trace the multi-phase documentation workflow: how raw source code is acquired, parsed into a navigable index, filtered by the scanner, and routed through LLM abstractions. Look for streaming implementations, cost-tracking mechanisms, and how structured prompts are assembled for codebase understanding.

## Step 7: Retrieval, Caching & Export Utilities (~15 min)

**Files:** [`crates/cache/Cargo.toml`](cache), [`crates/graph/Cargo.toml`](graph), [`crates/export/Cargo.toml`](export)

Evaluate supporting infrastructure that optimizes performance and output quality. Focus on TF-IDF snippet retrieval for context-aware queries, TTL-based caching strategies to reduce redundant LLM calls, dependency graph analysis for architectural insights, and final static format generation.

## Tips

- Use the PageRank rankings to prioritize debugging; higher-ranked files handle more cross-cutting concerns and shared state.
- The architecture separates heavy LLM processing from the UI; trace async boundaries and streaming endpoints to avoid blocking the main thread.
- Check .env.example early to avoid runtime failures when testing locally, especially for LLM provider keys and cache directory permissions.
