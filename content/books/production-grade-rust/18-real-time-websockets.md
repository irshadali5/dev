---
title: "Real-time WebSockets"
description: "HTTP is stateless and request-response oriented. But what if the admin dashboard needs real-time live analytics of users opening the newsletter? We use WebSockets."
order: 18
---

HTTP is stateless and request-response oriented. But what if the admin dashboard needs real-time live analytics of users opening the newsletter? We use **WebSockets**.

## Axum WS Upgrades

Axum makes it surprisingly easy to upgrade an HTTP route into a WebSocket stream.

**`src/routes/analytics.rs`**

```rust
use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg { msg } else { return; };

        if socket.send(Message::Text(format!("Live count: 1245"))).await.is_err() {
            break; // Client disconnected
        }
    }
}
```

In production, you'd use a broadcast channel (`tokio::sync::broadcast`) so a single worker parsing open-tracking webhooks can broadcast the event to thousands of connected WebSocket clients simultaneously.

  
> [!IMPORTANT]  
> Raw WebSockets are chaotic. We tame them by implementing the Actor Model (using actix or pure Tokio mpsc channels). Each connected client is an independent Actor with its own inbox. Crucially, we implement Backpressure. If a client has a slow connection and cannot consume the analytics firehose fast enough, their channel buffer fills up. We detect this and intelligently drop older, less important events (like scroll metrics) while preserving critical events, rather than crashing the server due to infinite memory buffering.