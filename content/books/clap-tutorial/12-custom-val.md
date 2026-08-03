---
title: "Custom Validators"
description: ""
---




## Custom Validators

Write any function `fn(&str) -> Result<T, E>` and pass it to
`value_parser`.



```rust

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    s.split_once('=')
     .map(|(k, v)| (k.to_string(), v.to_string()))
     .ok_or_else(|| format!("expected KEY=VALUE, got '{s}'"))
}

#[derive(Parser)]
struct Cli {
    /// Extra headers as KEY=VALUE
    #[arg(long, value_parser = parse_key_val)]
    header: Vec<(String, String)>,
}

```


\$ ./myapp --header
Content-Type=json --header X-Token=abc123 
