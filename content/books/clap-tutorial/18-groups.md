---
title: "Argument Groups"
description: ""
---




## Argument Groups

Argument groups let you say "exactly one of these must be provided" or
"at most one of these can be provided" across multiple arguments.



```rust

use clap::{Parser, ArgGroup};

#[derive(Parser)]
#[command(group(
    ArgGroup::new("auth")
        .required(true)       // one of these must be given
        .args(["api_key", "token"])
))]
struct Cli {
    /// Authenticate with API key
    #[arg(long)]
    api_key: Option<String>,

    /// Authenticate with bearer token
    #[arg(long)]
    token: Option<String>,
}

```


\$ ./myapp  
<span class="err">error: the following required arguments were not
provided:  
<span class="err"> <--api-key <API_KEY>\|--token <TOKEN>>
