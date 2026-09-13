# frontend

> Provides the interactive web interface for initiating codebase scans, viewing generated wiki documentation, and querying the LLM assistant.

The frontend is a React + Vite application that serves as the user-facing layer for the documentation generator. It handles project initialization, streams real-time scan progress, renders structured wiki content with syntax highlighting and diagram support, and provides an AI chat interface. State management is centralized via Zustand, while all backend communication is abstracted through a dedicated API module supporting both REST and streaming endpoints.

## Files

### `frontend/package.json`

Declares dependencies and build scripts for the Vite/React/TypeScript stack.

### `frontend/tsconfig.json`

Configures TypeScript compilation targets, strictness, and path resolution.

### `frontend/vite.config.ts`

Sets up Vite bundling with React Fast Refresh and Tailwind CSS processing.

### `frontend/src/App.tsx`

Configures client-side routing to dispatch users to the home, wiki, or chat views.

### `frontend/index.html`

Static entry point that mounts the React root container.

### `frontend/package-lock.json`

Locks dependency versions for reproducible builds.

### `frontend/src/main.tsx`

Bootstraps the React application and attaches it to the DOM.

### `frontend/src/pages/Home.tsx`

Landing view for triggering repository scans. Manages scan initiation and settings modal visibility.

- `handleScan` (function) - Initiates the backend codebase scan process and updates progress state.

### `frontend/src/pages/WikiView.tsx`

Main documentation viewer. Orchestrates sidebar navigation and content rendering based on selected pages.

### `frontend/src/pages/ChatView.tsx`

AI interaction panel. Streams LLM responses and maintains conversation context.

- `handleSend` (function) - Processes user input and triggers the streaming chat endpoint.

### `frontend/src/components/MermaidDiagram.tsx`

Wraps Mermaid.js to render flowcharts and architecture diagrams embedded in wiki markdown.

- `Props` (interface) - Defines expected attributes for the diagram wrapper component.

### `frontend/src/components/SettingsModal.tsx`

Overlay UI for configuring generation parameters. Syncs settings to the global store.

- `Props` (interface) - Defines configuration options and toggle state passed to the modal.

### `frontend/src/components/WikiContent.tsx`

Core renderer for wiki pages. Parses markdown, isolates Mermaid blocks, and safely converts text to HTML.

- `Props` (interface) - Defines content and styling attributes for the renderer.
- `ContentPart` (interface) - Represents a parsed segment of markdown, distinguishing between text and diagram blocks.
- `splitMermaid` (function) - Extracts Mermaid code blocks from raw markdown strings.
- `markdownToHtml` (function) - Converts processed markdown segments into sanitized HTML.
- `escapeHtml` (function) - Sanitizes plain text to prevent XSS when rendering untrusted wiki content.

### `frontend/src/components/WikiSidebar.tsx`

Left-panel navigation component displaying the generated wiki tree structure.

- `Props` (interface) - Defines navigation item attributes and selection handlers.

### `frontend/src/index.css`

Global stylesheet applying Tailwind utilities and base resets.

### `frontend/src/lib/api.ts`

Abstraction layer for backend communication. Defines request/response types and implements fetch/WebSocket calls.

- `getHeaders` (function) - Constructs authentication and content-type headers for API requests.
- `scanProject` (function) - Triggers the initial codebase analysis job on the backend.
- `streamScanProgress` (function) - Establishes a WebSocket connection to receive real-time scan logs.
- `getWiki` (function) - Fetches the complete wiki structure and metadata for a project.
- `getPage` (function) - Retrieves the raw markdown content for a specific wiki page.
- `getFileContent` (function) - Fetches raw source file contents linked from the wiki.
- `streamChat` (function) - Sends user queries to the LLM and streams incremental responses.

### `frontend/src/stores/wiki.ts`

Zustand-based global state manager. Tracks scan progress, wiki metadata, chat history, and UI toggles.

- `ChatReference` (interface) - Defines metadata linking a chat message to a specific wiki page or code artifact.
- `ChatMessage` (interface) - Represents a single turn in the AI conversation history.
- `WikiStore` (interface) - Defines the shape of the global Zustand store including scan status, wiki data, and chat state.

### `frontend/src/vite-env.d.ts`

Type declarations for Vite-specific environment variables and asset imports.

## Key Concepts

- **Streaming Communication**: Uses WebSocket/SSE for real-time scan progress and chat responses to prevent UI blocking and provide immediate feedback.
- **Centralized State Management**: Zustand handles cross-cutting concerns like scan status, wiki metadata, and chat history without prop drilling across the component tree.
- **Markdown-to-HTML Pipeline**: Custom parsing isolates Mermaid syntax before conversion to ensure safe rendering, proper diagram integration, and XSS prevention.
- **Component Orchestration**: Pages act as containers delegating rendering to specialized components (Sidebar, Content, Diagram) driven by shared state and routed URLs.

## Internal Relationships

- `frontend/src/App.tsx` → `frontend/src/pages/Home.tsx`: Routing configuration maps the root path to the landing view.
- `frontend/src/App.tsx` → `frontend/src/pages/WikiView.tsx`: Routing configuration maps the wiki path to the documentation viewer.
- `frontend/src/App.tsx` → `frontend/src/pages/ChatView.tsx`: Routing configuration maps the chat path to the AI assistant view.
- `frontend/src/components/SettingsModal.tsx` → `frontend/src/stores/wiki.ts`: Modal updates global configuration state via Zustand actions.
- `frontend/src/components/WikiContent.tsx` → `frontend/src/components/MermaidDiagram.tsx`: Content parser delegates extracted diagram blocks to this component for rendering.
- `frontend/src/main.tsx` → `frontend/src/App.tsx`: Entry point mounts the router and application shell into the DOM.
- `frontend/src/pages/ChatView.tsx` → `frontend/src/lib/api.ts`: Component sends chat payloads to backend and receives streamed responses.
- `frontend/src/pages/ChatView.tsx` → `frontend/src/stores/wiki.ts`: Component persists message history and references in global state.
- `frontend/src/pages/Home.tsx` → `frontend/src/lib/api.ts`: Component initiates scan jobs and subscribes to progress streams.
- `frontend/src/pages/Home.tsx` → `frontend/src/stores/wiki.ts`: Component tracks scan completion status and project metadata.
- `frontend/src/pages/Home.tsx` → `frontend/src/components/SettingsModal.tsx`: Component controls modal visibility and passes configuration props.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/lib/api.ts`: Component fetches wiki structure and page content on mount or navigation.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/stores/wiki.ts`: Component syncs active page selection and sidebar state.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/components/WikiSidebar.tsx`: Page passes wiki tree data and selection handlers to the sidebar.
- `frontend/src/pages/WikiView.tsx` → `frontend/src/components/WikiContent.tsx`: Page passes raw markdown and file paths to the content renderer.
