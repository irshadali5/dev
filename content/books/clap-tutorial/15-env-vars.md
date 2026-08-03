---
title: "Environment Variables"
description: ""
---




## Environment Variables

Add `features = ["env"]` to your Cargo.toml, then use
`#[arg(env = "VAR_NAME")]`. CLI flags take precedence over env vars.



```toml

clap = { version = "4", features = ["derive", "env"] }

```



```rust

#[derive(Parser)]
struct Cli {
    /// Database URL — reads $DATABASE_URL if not set on CLI
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// API key — hides value in help output
    #[arg(long, env = "API_KEY", hide_env_values = true)]
    api_key: String,
}

```


\$
DATABASE_URL=postgres://localhost/db ./myapp  
<span class="cm">\# No --database-url needed — reads from env  
  
\$ ./myapp --database-url
postgres://prod/db  
<span class="cm">\# CLI flag overrides env var 

Help output shows where a value came from:


 --database-url <DATABASE_URL> [env:
DATABASE_URL=postgres://localhost/db] 
