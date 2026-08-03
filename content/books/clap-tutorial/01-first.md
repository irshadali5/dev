---
title: "Your First clap Program"
description: ""
---

first">

<div class="section-eyebrow">

Chapter 2** >

## Your First clap Program

The derive approach: you define a **struct** and annotate it with
`#[derive(Parser)]`. clap reads those annotations and builds the entire
parser from your struct's shape.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>
<span class="code-filename">src/main.rs</span>

copy

** >

    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(name = "greet", about = "A greeting CLI")]
    struct Cli {
        /// Name of the person to greet
        name: String,
    }

    fn main() {
        let args = Cli::parse();
        println!("Hello, {}!", args.name);
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">cargo run --
irshad</span>  
<span class="out">Hello, irshad!</span>  
  
<span class="prompt">\$</span> <span class="cmd">cargo run --
--help</span>  
<span class="out">A greeting CLI</span>  
  
<span class="out">Usage: greet <NAME></span>  
  
<span class="out">Arguments:</span>  
<span class="out"> <NAME> Name of the person to greet</span>  
  
<span class="out">Options:</span>  
<span class="out"> -h, --help Print help</span>  
<span class="out"> -V, --version Print version</span> ** >



> [!TIP] > **Key insight** >

The `/// doc comment` on each field becomes the help text automatically.
This is not magic — clap's proc macro reads it at compile time. Always
write doc comments on your fields.

** >

### What just happened?

Three things the macro wired up for you:

| You wrote | clap did |
|----|----|
| `struct Cli { name: String }` | Registered a positional argument called NAME |
| `#[command(about = "...")]` | Set the help description line |
| `Cli::parse()` | Parsed argv, validated types, printed errors & exited on failure |