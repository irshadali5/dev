---
title: "ValueEnum — Restrict to Known Values"
description: ""
---




## ValueEnum — Restrict to Known Values

Use `#[derive(ValueEnum)]` on an enum to restrict an argument to a fixed
set of values. clap generates the valid options list in help text
automatically.



```rust

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Toml,
    Yaml,
    #[value(name = "plain-text")]  // custom CLI name
    PlainText,
}

#[derive(Parser)]
struct Cli {
    /// Output format [json, toml, yaml, plain-text]
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,
}

fn main() {
    let args = Cli::parse();
    match args.format {
        OutputFormat::Json      => println!("outputting JSON"),
        OutputFormat::Toml      => println!("outputting TOML"),
        OutputFormat::Yaml      => println!("outputting YAML"),
        OutputFormat::PlainText => println!("outputting plain text"),
    }
}

```


\$ ./myapp --format
toml  
outputting TOML  
  
\$ ./myapp --format
xml  
<span class="err">error: invalid value 'xml' for '--format
<FORMAT>'  
<span class="err"> [possible values: json, toml, yaml,
plain-text] 
