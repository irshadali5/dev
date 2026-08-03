---
title: "Argument Groups"
description: ""
---

groups">

<div class="section-eyebrow">

Chapter 19** >

## Argument Groups

Argument groups let you say "exactly one of these must be provided" or
"at most one of these can be provided" across multiple arguments.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

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

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp</span>  
<span class="err">error: the following required arguments were not
provided:</span>  
<span class="err"> <--api-key <API_KEY>\|--token <TOKEN>></span>
** >