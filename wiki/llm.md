# llm

> Provides a unified abstraction layer for interacting with OpenAI-compatible LLM APIs, handling request routing, streaming, cost tracking, and generating structured prompts for codebase documentation.

Encapsulates all LLM interactions required to analyze source code and generate wiki documentation. It abstracts HTTP client configuration, API key validation, and model-specific endpoint resolution while enforcing consistent message formatting and response parsing. The module separates transport logic from prompt engineering, allowing downstream components to request structured outputs without managing API details or context assembly. It also implements built-in token and cost accounting to prevent runaway usage during bulk analysis.

## Files

### `crates/llm/Cargo.toml`

Defines crate metadata and runtime dependencies including reqwest for HTTP, serde for serialization, thiserror for error handling, and regex for output parsing.

### `crates/llm/src/client.rs`

Implements the HTTP client wrapper for LLM APIs. Manages synchronous completions, server-sent event streaming, dynamic endpoint/model resolution, and cumulative token/cost tracking.

- `LLMClient` (struct) - Core wrapper that holds reqwest client state, model configuration, API credentials, and running totals for tokens and cost.
- `complete` (method) - Sends a Vec<ChatMessage> to the API and deserializes the full response into a ChatResponse struct.
- `stream` (method) - Initiates a streaming request, yielding incremental StreamChunk events as the LLM generates tokens.
- `resolve_api_base` (function) - Normalizes or defaults the API endpoint URL based on the provided model and explicit override.
- `ChatMessage` (struct) - Standardized data contract representing role/content pairs for system, user, and assistant messages.

### `crates/llm/src/lib.rs`

Entry point that re-exports public types and functions from client and prompts to expose a clean module API.

### `crates/llm/src/prompts.rs`

Constructs task-specific prompt chains using predefined system instructions and regex-based JSON extraction. Ensures LLM outputs conform to expected schemas for automated parsing.

- `build_overview_prompt` (function) - Assembles initial context messages using file tree, key files, and language to generate high-level documentation.
- `build_module_prompt` (function) - Constructs focused prompts for analyzing individual modules or directories.
- `build_architecture_prompt` (function) - Generates prompts targeting cross-module relationships and system design patterns.
- `extract_json` (function) - Sanitizes raw LLM text output using regex to isolate and parse valid JSON payloads.
- `build_chat_prompt` (function) - Formats conversational history and current queries for interactive documentation assistance.

## Key Concepts

- **Unified API Abstraction**: Hides provider-specific HTTP quirks behind a consistent LLMClient interface, enabling easy model swapping without refactoring downstream logic.
- **Deterministic Output Parsing**: Regex-based extract_json sanitizes raw LLM text into valid serde_json::Value, preventing parser failures from markdown wrappers or trailing text.
- **Cumulative Usage Accounting**: LLMClient tracks input/output tokens and estimated costs across all requests to monitor budget and detect anomalies during bulk codebase scanning.
- **Context-Aware Prompt Routing**: Separate builder functions map specific documentation tasks to optimized system instructions and message structures, improving output relevance and reducing token waste.

## Internal Relationships

- `prompts.rs` → `client.rs`: Prompt builders return Vec<ChatMessage> which are directly consumed by LLMClient::complete and LLMClient::stream for execution.
- `lib.rs` → `client.rs`: Aggregates internal modules and exposes them as a single public namespace for downstream crates.
- `lib.rs` → `prompts.rs`: Exports prompt construction utilities alongside the client wrapper to provide a complete LLM interaction surface.
- `External callers` → `llm module`: Documentation generators invoke prompt builders to assemble context, then pass them to LLMClient to fetch structured analysis.
