# frontend

> Web interface for initiating codebase scans, viewing generated documentation, and interacting with an AI chat assistant.

A React-based single-page application that serves as the user-facing layer for the wiki documentation generator. It manages the full lifecycle of codebase indexing, displays the resulting hierarchical documentation with safe markdown rendering and diagram support, and provides a real-time chat interface for querying indexed knowledge. The module centralizes network communication, maintains global application state, and optimizes client-side rendering to handle large documentation sets without overwhelming the browser or LLM token limits.

## Files

### `frontend/package.json`

Dependency manifest declaring runtime and build requirements for Vite, React, Tailwind, Mermaid, Zustand, and related tooling.

### `frontend/tsconfig.json`

TypeScript compiler configuration enforcing strict typing, module resolution, and JSX transformation rules.

### `frontend/vite.config.ts`

Build pipeline configuration enabling React Fast Refresh, Tailwind CSS processing, and optimized production bundling.

### `frontend/src/App.tsx`

Root router component mapping URL paths to Home, WikiView, and ChatView pages to enable client-side navigation.

### `frontend/index.html`

Static entry point that injects the compiled JavaScript bundle and sets up the DOM root.

### `frontend/package-lock.json`

Deterministic dependency tree snapshot ensuring reproducible builds across environments.

### `frontend/src/components/MermaidDiagram.tsx`

Isolated renderer for Mermaid.js flowcharts and diagrams extracted from wiki markdown to prevent blocking the main thread.

- `Props` (interface) - Defines expected props for diagram rendering including content and container dimensions.

### `frontend/src/components/SettingsModal.tsx`

Overlay UI for adjusting scan parameters, repository URLs, and backend connection settings.

- `Props` (interface) - Controls modal visibility and callback handlers for saving configuration changes.

### `frontend/src/components/WikiContent.tsx`

Primary documentation renderer that parses raw markdown, isolates Mermaid blocks, sanitizes HTML, and formats text for safe display.

- `Props` (interface) - Passes markdown string and styling options to the content renderer.
- `ContentPart` (interface) - Discriminated union type distinguishing between standard markdown text and Mermaid diagram blocks.
- `splitMermaid` (function) - Parses input markdown string and separates standard text from fenced Mermaid code blocks.
- `markdownToHtml` (function) - Converts standard markdown segments into sanitized HTML for rendering.
- `escapeHtml` (function) - Strips dangerous characters from markdown content to prevent XSS attacks.

### `frontend/src/components/WikiSidebar.tsx`

Navigable tree component displaying the generated wiki file hierarchy and handling page selection clicks.

- `Props` (interface) - Provides tree data structure, active page ID, and selection callback handler.

### `frontend/src/index.css`

Global stylesheet importing Tailwind directives and applying base reset styles.

### `frontend/src/lib/api.ts`

Centralized HTTP client defining request/response schemas and implementing async functions for scanning, progress tracking, wiki retrieval, and chat streaming.

- `getHeaders` (function) - Attaches authentication tokens and content-type headers to outgoing requests.
- `ScanRequest` (interface) - Schema for initiating a new codebase indexing job.
- `ProjectInfo` (interface) - Metadata returned after successful project scan completion.
- `WikiStructure` (interface) - Hierarchical representation of the generated documentation tree.
- `SidebarItem` (interface) - Individual node definition for sidebar navigation components.
- `PageMeta` (interface) - Lightweight metadata for wiki pages used in navigation and previews.
- `WikiPage` (interface) - Full document payload containing markdown content and associated metadata.
- `scanProject` (function) - Triggers the backend indexing pipeline and returns the resulting project identifier.
- `streamScanProgress` (function) - Establishes a streaming connection to monitor real-time indexing status and logs.
- `getWiki` (function) - Fetches the complete documentation tree structure for a given project.
- `getPage` (function) - Retrieves the full markdown content for a specific wiki page by ID.
- `getFileContent` (function) - Downloads raw source file content referenced within the documentation.
- `ChatTurn` (interface) - Schema representing a single message exchange in the AI chat session.
- `streamChat` (function) - Sends user queries to the LLM and yields incremental response chunks via stream.

### `frontend/src/main.tsx`

Application bootstrap script initializing the React root and mounting the App component to the DOM.

### `frontend/src/pages/ChatView.tsx`

Interactive chat interface handling message composition, API streaming integration, and state synchronization.

- `handleSend` (function) - Processes user input, appends it to the store, triggers the streaming API call, and manages loading states.

### `frontend/src/pages/Home.tsx`

Dashboard page for triggering new scans, monitoring progress, and managing project settings via modal integration.

- `handleScan` (function) - Validates form inputs, calls the scan API, updates global store with progress, and redirects upon completion.

### `frontend/src/pages/WikiView.tsx`

Documentation viewer page orchestrating sidebar navigation and content rendering based on selected page IDs.

### `frontend/src/stores/wiki.ts`

Zustand-based global state store managing chat history, wiki metadata, loading flags, and active selections without prop drilling.

- `ChatReference` (interface) - Tracks context snippets or source links attached to AI chat responses.
- `ChatMessage` (interface) - Represents a complete chat turn including role, content, and optional references.
- `WikiStore` (interface) - Defines the shape of the global state slice including selectors, setters, and initialization logic.

### `frontend/src/vite-env.d.ts`

Type declarations providing IDE autocompletion for Vite-specific environment variables and static asset imports.

## Key Concepts

- **Zustand Global State**: Eliminates prop drilling by centralizing chat history, wiki metadata, and UI flags in a lightweight store, enabling instant cross-component synchronization.
- **Streaming API Integration**: Handles long-running operations like scan progress and LLM responses via fetch streams, providing real-time feedback without blocking the UI thread.
- **Client-Side Markdown & Diagram Parsing**: Safely processes raw markdown on the frontend, isolating Mermaid blocks to prevent XSS and rendering heavy visualizations asynchronously to maintain performance.
- **SPA Routing Architecture**: Uses react-router-dom to manage client-side navigation between dashboard, documentation viewer, and chat interfaces, preserving application state and avoiding full page reloads.

## Internal Relationships

- `frontend/src/App.tsx` → `frontend/src/pages/Home.tsx`: Registers the Home page route for initial landing and scan initiation.
- `frontend/src/App.tsx` → `frontend/src/pages/WikiView.tsx`: Registers the WikiView route for documentation browsing.
- `frontend/src/App.tsx` → `frontend/src/pages/ChatView.tsx`: Registers the ChatView route for AI-assisted querying.
- `frontend/src/components/SettingsModal.tsx` → `frontend/src/stores/wiki.ts`: Reads and writes configuration state to persist user preferences across sessions.
- `frontend/src/components/WikiContent.tsx` → `frontend/src/components/MermaidDiagram.tsx`: Delegates extracted diagram blocks to the isolated renderer for safe, non-blocking visualization.
- `frontend/src/main.tsx` → `frontend/src/App.tsx`: Mounts the React application tree to the DOM element defined in index.html.
- `frontend/src/pages/ChatView.tsx` → `frontend/src/lib/api.ts`: Invokes streaming endpoints to send queries and receive incremental LLM responses.
- `frontend/src/pages/ChatView.tsx` → `frontend/src/stores/wiki.ts`: Dispatches actions to update chat history, loading states, and reference metadata.
- `frontend/src/pages/Home.tsx` → `frontend/src/lib/api.ts`: Calls scan initiation and progress streaming endpoints to drive the indexing workflow.
- `frontend/src/pages/Home.tsx` → `frontend/src/stores/wiki.ts`: Updates global state with project metadata, scan progress, and completion flags.
- `frontend/src/pages/Home.tsx` → `frontend/src/components/SettingsModal.tsx`: Opens and controls the settings overlay to adjust scan parameters before execution.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/lib/api.ts`: Fetches wiki structure and individual page content based on user navigation.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/stores/wiki.ts`: Syncs selected page ID and loading states with the global store for cross-component consistency.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/components/WikiSidebar.tsx`: Passes tree data and selection callbacks to render the navigation panel.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/components/WikiContent.tsx`: Injects fetched markdown content into the renderer for display.
