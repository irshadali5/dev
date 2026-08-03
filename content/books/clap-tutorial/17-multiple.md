---
title: "Multiple Values"
description: ""
---




## Multiple Values

Three ways to accept multiple values for one argument:



```rust

#[derive(Parser)]
struct Cli {
    /// 1. Repeated flags: --tag a --tag b --tag c
    #[arg(long)]
    tag: Vec<String>,

    /// 2. Space-separated after flag: --file a.txt b.txt c.txt
    #[arg(long, num_args = 1..)]
    file: Vec<String>,

    /// 3. Exactly N values: --coords 10 20
    #[arg(long, num_args = 2)]
    coords: Vec<f64>,
}

```


\$ ./myapp --tag rust
--tag cli --tag cool  
\$ ./myapp --file a.txt
b.txt c.txt  
\$ ./myapp --coords 12.5
-3.7 
