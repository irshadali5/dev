---
title: "AI Integration (Gemini 3.1 Pro)"
description: "Modern applications increasingly rely on AI to enhance the user experience. We'll augment our email newsletter API by integrating Gemini 3.1 Pro to automatically generate catchy subject lines and summarize long newsletter content."
order: 15
---

Modern applications increasingly rely on AI to enhance the user experience. We'll augment our email newsletter API by integrating **Gemini 3.1 Pro** to automatically generate catchy subject lines and summarize long newsletter content.

## API Key Configuration

First, ensure your `.env` file includes the Gemini API key:

**`.env`**

```env
GEMINI_API_KEY="your_api_key_here"
```

We use `config` to securely load this into our state, making it available to all Axum route handlers.

## Calling the LLM with Reqwest

We build a dedicated client for Gemini. We use `reqwest` to make the HTTP request and `serde` for parsing the JSON response.

**`src/ai_client.rs`**

```rust
use reqwest::Client;
use secrecy::{Secret, ExposeSecret};
use serde_json::json!;

pub struct GeminiClient {
    pub client: Client,
    pub api_key: Secret<String>,
}

impl GeminiClient {
    pub async fn generate_subject(&self, content: &str) -> Result<String, reqwest::Error> {
        let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-pro:generateContent";
        
        let body = json!({
            "contents": [{
                "parts": [{ "text": format!("Generate a catchy subject line for this newsletter: {}", content) }]
            }]
        });

        let response = self.client
            .post(url)
            .header("x-goog-api-key", self.api_key.expose_secret())
            .json(&body)
            .send()
            .await?;

        // Parse the JSON to extract the response text
        // Implementation omitted for brevity
        Ok("Extracted Subject Line".to_string())
    }
}
```

  
> [!WARNING]  
> When depending on external services, you must handle timeouts and rate-limiting gracefully. Use tower::timeout and retry middleware to ensure your application doesn't hang indefinitely.  

  
> [!IMPORTANT]  
> Waiting for the LLM to generate a 5-paragraph newsletter summary causes unacceptable HTTP timeouts. We migrate our Gemini API endpoint to use Server-Sent Events (SSE). Using Axum's Sse response type and tokio_stream, we pipe the chunked stream from the reqwest Gemini response directly through to the client browser in real-time. This provides the "typing" effect seen in ChatGPT, dropping perceived latency from 15 seconds to 200 milliseconds.