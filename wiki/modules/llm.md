# llm

> Provides a unified interface for interacting with external LLM APIs while constructing domain-specific prompts for automated codebase documentation generation.

The llm module abstracts HTTP communication, request/response parsing, and streaming handling for OpenAI-compatible endpoints. It centralizes prompt engineering for documentation tasks and enforces structured output by extracting JSON from raw LLM responses. This reduces boilerplate across the application, ensures consistent API integration, and supports cost tracking to align with the project's goal of minimizing token expenditure during documentation generation.

## Files

### `crates/llm/Cargo.toml`

Defines crate metadata and declares runtime dependencies for HTTP requests, serialization, error handling, and pattern matching.

### `crates/llm/src/client.rs`

Manages network requests, models API payloads and responses, tracks token usage and costs, and resolves endpoint configurations.

- `LLMError` (enum) - Standardized error type for API failures and parsing issues.
- `ChatMessage` (struct) - Represents role/content pairs used in conversation history and prompts.
- `LLMClient` (struct) - Core client wrapper that holds configuration, tracks cumulative token usage/cost, and dispatches requests.
- `complete` (function) - Sends a prompt sequence and returns a single complete response.
- `stream` (function) - Sends a prompt sequence and yields an async iterator of response chunks.
- `resolve_api_base` (function) - Determines the correct endpoint URL based on model and key heuristics.
- `resolve_model_name` (function) - Normalizes model identifiers to match provider expectations.

### `crates/llm/src/lib.rs`

Module entry point that re-exports public types and functions from client and prompts submodules.

### `crates/llm/src/prompts.rs`

Constructs task-specific prompt templates and parses LLM outputs into structured JSON for downstream consumption.

- `build_overview_prompt` (function) - Generates system and user messages for high-level codebase summarization.
- `build_module_prompt` (function) - Creates prompts for analyzing individual modules or directories.
- `build_architecture_prompt` (function) - Constructs prompts focused on cross-module relationships and system design.
- `build_reading_guide_prompt` (function) - Produces prompts that generate navigational instructions for developers.
- `extract_json` (function) - Applies regex filtering to isolate valid JSON blocks from raw LLM text.

## Key Concepts

- **Structured Prompt Engineering**: Prompts are generated programmatically with strict language and formatting instructions to ensure predictable, machine-parseable documentation output.
- **Cost & Usage Tracking**: LLMClient accumulates prompt and completion tokens per request, calculating total session cost to monitor and minimize API spending.
- **Streaming vs Complete Modes**: Supports both synchronous full-response and asynchronous chunked-streaming calls to handle varying output lengths and latency requirements.
- **JSON Enforcement**: Raw LLM text is filtered through regex-based extraction to guarantee valid JSON payloads for downstream indexing and wiki generation.

## Internal Relationships

- `lib.rs` → `client.rs`: Aggregates and re-exports the LLMClient and related types for external use.
- `lib.rs` → `prompts.rs`: Aggregates and re-exports prompt builders and JSON extraction utilities.
- `prompts.rs` → `client.rs`: Uses ChatMessage structs to assemble prompt sequences that are passed directly to LLMClient methods.
- `client.rs` → `External LLM APIs`: Dispatches HTTP POST requests with serialized prompts and consumes streaming or complete JSON responses.
