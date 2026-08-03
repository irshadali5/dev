---
title: "The LLM Gateway (Multi-Provider AI)"
description: "Vendor lock-in is a massive risk in the fast-paced AI ecosystem. If OpenAI experiences an outage, or if Gemini releases a vastly superior and cheaper model overnight, our application must be able to pivot instantly without rewriting business logic. We achieve this by building an LLM Gateway Architecture."
order: 20
---

Vendor lock-in is a massive risk in the fast-paced AI ecosystem. If OpenAI experiences an outage, or if Gemini releases a vastly superior and cheaper model overnight, our application must be able to pivot instantly without rewriting business logic. We achieve this by building an **LLM Gateway Architecture**.

## The Provider Trait

We define a core domain trait that abstracts away the specific HTTP clients of any given AI provider. We use `Native Async Traits (Rust 1.75+)` (or native async traits in Rust 1.75+) to enforce a uniform interface.

**`src/ai/gateway.rs`**

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}

#[trait_variant::make(LlmProvider: Send)]
pub trait LocalLlmProvider {
    async fn generate_text(&self, messages: &[AiMessage]) -> Result<String, GatewayError>;
    async fn generate_stream(&self, messages: &[AiMessage]) -> Result<BoxStream<String>, GatewayError>;
}
```

## Dynamic Fallback Routing

We implement adapters for **OpenAI**, **Anthropic**, **Gemini**, and even local instances like **Ollama** or **vLLM**. Our Gateway orchestrator holds a list of these providers, prioritized by cost or performance.

**`src/ai/orchestrator.rs`**

```rust
pub struct AiOrchestrator {
    providers: Vec<Arc<dyn LlmProvider>>,
}

impl AiOrchestrator {
    pub async fn execute_with_failover(&self, msgs: &[AiMessage]) -> Result<String, GatewayError> {
        let mut last_error = None;
        
        for provider in &self.providers {
            match provider.generate_text(msgs).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    tracing::warn!("Provider failed, failing over to next: {:?}", e);
                    last_error = Some(e);
                }
            }
        }
        Err(last_error.unwrap_or(GatewayError::NoProvidersAvailable))
    }
}
```

  
> [!IMPORTANT]  
> Using dyn LlmProvider introduces a vtable lookup penalty at runtime. While negligible for HTTP calls, in hyperscale environments executing millions of local inferences via vLLM, we replace dynamic dispatch with a massive Enum (Static Dispatch). Furthermore, we implement an atomic Token Budgeting middleware that strictly terminates streams if a runaway model exceeds the allocated max_tokens, preventing billing catastrophes.  

## Hyper-Expansion: Server-Sent Events (SSE) Streaming Pipeline

To implement real-time streaming from Gemini to the client, we bridge the `reqwest::Response` stream to an Axum `Sse` stream.

**`src/ai/streaming.rs`**

```rust
use axum::response::sse::{Event, Sse};
use axum::extract::State;
use futures::stream::{Stream, StreamExt};
use std::convert::Infallible;
use serde_json::Value;

pub async fn generate_newsletter_stream(
    State(ai_client): State<Arc<GeminiClient>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    
    let reqwest_response = ai_client.start_stream_request("Write a newsletter").await.unwrap();
    let byte_stream = reqwest_response.bytes_stream();
    
    let sse_stream = byte_stream
        .filter_map(|chunk| async move {
            match chunk {
                Ok(bytes) => {
                    if let Ok(json) = serde_json::from_slice::<Value>(&bytes) {
                        let text = json["candidates"][0]["content"]["parts"][0]["text"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        Some(Ok(Event::default().data(text)))
                    } else { None }
                }
                Err(_) => None
            }
        });

    Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(1))
            .text("keep-alive-text")
    )
}
```

### Stream Pipeline Architecture

```mermaid
sequenceDiagram autonumber participant C as Client (Browser) participant A as Axum Server (Sse) participant R as Reqwest participant G as Gemini API C->>A: GET /api/generate (SSE Request) A->>R: POST /v1beta/models/gemini:streamGenerateContent R->>G: Initiate TCP/TLS connection loop Streaming Response G-->>R: TCP Packet [Bytes] R-->>A: Yield chunked Bytes A->>A: Deserialize to JSON, extract Token A-->>C: SSE Event: "data: token\\n\\n" end G-->>R: EOF R-->>A: Stream End A-->>C: HTTP Close
```