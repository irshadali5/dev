---
title: "ValueEnum — Restrict to Known Values"
description: ""
---

value-enum">

<div class="section-eyebrow">

Chapter 15** >

## ValueEnum — Restrict to Known Values

Use `#[derive(ValueEnum)]` on an enum to restrict an argument to a fixed
set of values. clap generates the valid options list in help text
automatically.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

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

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp --format
toml</span>  
<span class="out">outputting TOML</span>  
  
<span class="prompt">\$</span> <span class="cmd">./myapp --format
xml</span>  
<span class="err">error: invalid value 'xml' for '--format
<FORMAT>'</span>  
<span class="err"> [possible values: json, toml, yaml,
plain-text]</span> ** >