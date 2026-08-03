---
title: "Conflicts &amp; Requires"
description: ""
---




## Conflicts & Requires

Encode logical relationships between arguments so clap validates them
automatically.



```rust

#[derive(Parser)]
struct Cli {
    /// Quiet mode (conflicts with --verbose)
    #[arg(long, conflicts_with = "verbose")]
    quiet: bool,

    /// Verbose mode (conflicts with --quiet)
    #[arg(long, conflicts_with = "quiet")]
    verbose: bool,

    /// Output file (requires --format)
    #[arg(long, requires = "format")]
    output: Option<String>,

    /// Output format (required if --output is set)
    #[arg(long)]
    format: Option<String>,
}

```


\$ ./myapp --quiet
--verbose  
<span class="err">error: the argument '--quiet' cannot be used with
'--verbose'  
  
\$ ./myapp --output
result.txt ← missing --format  
<span class="err">error: the following required arguments were not
provided: --format <FORMAT> 
