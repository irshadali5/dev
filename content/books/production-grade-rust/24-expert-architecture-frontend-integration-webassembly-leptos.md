---
title: "Expert Architecture: Frontend Integration & WebAssembly (Leptos)"
description: "A backend doesn't exist in a vacuum. We must serve a blazing-fast admin dashboard. We leverage Leptos, a cutting-edge Rust web framework built on fine-grained reactivity (Signals). Leptos provides seamless Server-Side Rendering (SSR) and Isomorphic Server Functions that compile into native Axum routes on the server and fetch calls on the client."
order: 24
---

A backend doesn't exist in a vacuum. We must serve a blazing-fast admin dashboard. We leverage **Leptos**, a cutting-edge Rust web framework built on fine-grained reactivity (Signals). Leptos provides seamless Server-Side Rendering (SSR) and Isomorphic Server Functions that compile into native Axum routes on the server and fetch calls on the client.

## The Dashboard Component (Signals & Suspense)

**`frontend/src/dashboard.rs`**

```rust
use leptos::*;
use shared::DashboardStats;

#[server(GetDashboardStats, "/api/admin")]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    // This code ONLY runs on the server (Axum).
    // It directly queries the database pool injected into Axum's state.
    let pool = use_context::<sqlx::PgPool>().unwrap();
    let active_subscribers = sqlx::query_scalar!("SELECT count(*) FROM subscribers WHERE status = 'active'")
        .fetch_one(&pool).await.unwrap_or(0);
        
    Ok(DashboardStats {
        active_subscribers: active_subscribers as u64,
        emails_sent_today: 15420,
        bounce_rate: 0.02,
    })
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    // create_resource automatically handles SSR serialization and client hydration
    let stats = create_resource(|| (), |_| async move {
        get_dashboard_stats().await.unwrap()
    });

    view! {
        <div class="dashboard-container">
            <h1>"Admin Dashboard"</h1>
            <Suspense fallback=move || view! { <p>"Loading hyperscale analytics..."</p> }>
                {move || {
                    stats.get().map(|s| {
                        view! {
                            <div class="stats-grid">
                                <div class="stat-card">
                                    <h3>"Active Subscribers"</h3>
                                    <p>{s.active_subscribers}</p>
                                </div>
                            </div>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
```