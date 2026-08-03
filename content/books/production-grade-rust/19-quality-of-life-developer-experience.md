---
title: "Quality of Life & Developer Experience"
description: "Writing reliable code is only half the battle. A phenomenal Developer Experience (DX) ensures your team stays fast and happy."
order: 19
---

Writing reliable code is only half the battle. A phenomenal Developer Experience (DX) ensures your team stays fast and happy.

## Hot Reloading

Instead of manually recompiling, use `cargo-watch` to recompile and restart the server automatically:

**`terminal`**

```bash
cargo watch -x run
```

## Continuous Integration (CI/CD)

Your CI pipeline should strictly enforce quality. A robust GitHub Actions workflow should include:

*   `cargo fmt --check`: Enforce standard formatting.
*   `cargo clippy -- -D warnings`: Fail the build on any lint warnings.
*   `cargo nextest run`: Run unit and integration tests faster.
*   `cargo deny check`: Audit dependencies for vulnerabilities and incompatible licenses.
*   `sqlx prepare --check`: Verify that compile-time SQL queries match the database schema without needing a live DB during the build.

## Database Tooling

For visualizing data locally without endless `psql` commands, use tools like **DBeaver** or **TablePlus**. Connect them directly to your local Postgres Docker container to instantly see the results of your integration tests.

  
> [!IMPORTANT]  
> When boilerplate gets out of hand, we write our own Procedural Macros (using syn and quote). We write a custom #[derive(AutoLog)] macro that automatically injects tracing::instrument spans and input validation into our domain functions at compile time. This allows our engineers to write pure business logic without cluttering their functions with repetitive observability boilerplate, achieving the pinnacle of Developer Experience (DX).