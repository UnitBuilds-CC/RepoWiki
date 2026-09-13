# Reading Guide

Approach this codebase by tracing the data flow from configuration through backend analysis pipelines to the frontend interface. Start with project setup to understand dependencies and environment requirements, then move to entry points to see how execution begins. Next, examine core logic modules to grasp data modeling and analysis algorithms, followed by utility and integration layers like caching and exports. Finally, review the frontend to understand how state and API responses are rendered into interactive views.

## Step 1: Foundation & Configuration (~5 min)

**Files:** [`.env.example`](modules/root.md), [`Cargo.toml`](modules/root.md), [`README.md`](modules/root.md)

Identify required environment variables, dependency versions, and workspace structure. Look for how the Rust monorepo organizes crates, note the documented initialization workflow, and understand the baseline assumptions for building and running the system.

## Step 2: Entry Points & Application Bootstrapping (~5 min)

**Files:** [`frontend/src/App.tsx`](modules/frontend.md), [`crates/cli/Cargo.toml`](modules/cli.md)

Trace how the web application initializes routes and mounts root components. Examine the CLI crate configuration to understand terminal command routing, argument parsing, and how user input is dispatched to backend analysis pipelines.

## Step 3: Core Data Models & Structural Analysis (~10 min)

**Files:** [`crates/core/Cargo.toml`](modules/core.md), [`crates/graph/Cargo.toml`](modules/graph.md), [`crates/index/Cargo.toml`](modules/index.md)

Review strongly-typed structs and enums used for project scanning and indexing. Look for how the graph module traverses repositories to prioritize documentation targets, and examine how the index module parses source code into structured metadata for efficient LLM context retrieval.

## Step 4: LLM Orchestration, Caching & Export (~10 min)

**Files:** [`crates/analyzer/Cargo.toml`](modules/analyzer.md), [`crates/cache/Cargo.toml`](modules/cache.md), `crates/export/Cargo.toml`

Analyze how domain-specific prompts are constructed and routed to external APIs. Look for TTL-managed cache invalidation policies that prevent redundant processing, and examine incremental write optimizations used to generate Markdown, JSON, and HTML documentation outputs.

## Step 5: Frontend API Layer & Global State (~10 min)

**Files:** [`frontend/src/lib/api.ts`](modules/frontend.md), [`frontend/src/stores/wiki.ts`](modules/frontend.md)

Inspect HTTP request wrappers, error handling, and streaming progress tracking mechanisms. Study the reactive state store to see how wiki data, scan status, and chat history are synchronized, persisted, and consumed by UI components.

## Step 6: Interactive UI & Documentation Rendering (~15 min)

**Files:** [`frontend/src/pages/Home.tsx`](modules/frontend.md), [`frontend/src/pages/ChatView.tsx`](modules/frontend.md), [`frontend/src/pages/WikiView.tsx`](modules/frontend.md), [`frontend/src/components/MermaidDiagram.tsx`](modules/frontend.md), [`frontend/src/components/WikiContent.tsx`](modules/frontend.md), [`frontend/src/components/WikiSidebar.tsx`](modules/frontend.md), [`frontend/src/components/SettingsModal.tsx`](modules/frontend.md)

Map the user journey from the home dashboard to active chat sessions and wiki browsing. Focus on how Mermaid diagrams are dynamically rendered, how wiki content is parsed and displayed, sidebar navigation logic, and how settings modal state interacts with the global store.

## Tips

- Follow the data flow: start with how raw source code enters the scanner, moves through indexing, gets processed by the analyzer, and finally surfaces in the frontend store.
- Use the PageRank rankings as a priority guide; higher-ranked files typically handle more cross-cutting concerns or critical paths in the application.
- When debugging LLM-related issues, trace the prompt construction in the analyzer module alongside the cache TTL settings to identify redundancy or context window limits.
