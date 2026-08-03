---
title: "Multiple Values"
description: ""
---

multiple">

<div class="section-eyebrow">

Chapter 18** >

## Multiple Values

Three ways to accept multiple values for one argument:

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

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

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp --tag rust
--tag cli --tag cool</span>  
<span class="prompt">\$</span> <span class="cmd">./myapp --file a.txt
b.txt c.txt</span>  
<span class="prompt">\$</span> <span class="cmd">./myapp --coords 12.5
-3.7</span> ** >