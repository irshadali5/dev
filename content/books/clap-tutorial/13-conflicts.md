---
title: "Conflicts &amp; Requires"
description: ""
---

conflicts">

<div class="section-eyebrow">

Chapter 14** >

## Conflicts & Requires

Encode logical relationships between arguments so clap validates them
automatically.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

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

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp --quiet
--verbose</span>  
<span class="err">error: the argument '--quiet' cannot be used with
'--verbose'</span>  
  
<span class="prompt">\$</span> <span class="cmd">./myapp --output
result.txt</span><span class="out"> ← missing --format</span>  
<span class="err">error: the following required arguments were not
provided: --format <FORMAT></span> ** >