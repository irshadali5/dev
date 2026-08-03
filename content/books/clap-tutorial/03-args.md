---
title: "Arguments, Options, and Flags"
description: ""
---

args">

<div class="section-eyebrow">

Chapter 4** >

## Arguments, Options, and Flags

These three are the core building blocks of any CLI. The difference is
**how they're spelled on the command line**.

| Kind | Example on CLI | How it looks in struct |
|----|----|----|
| **Positional argument** | `myapp foo.txt` | Plain field, no `#[arg]` needed |
| **Option** (named, takes value) | `myapp --output out.txt` | `#[arg(short, long)]` on non-bool field |
| **Flag** (named, no value) | `myapp --verbose` | `#[arg(short, long)]` on a `bool` field |

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>
<span class="code-filename">src/main.rs — all three kinds</span>

copy

** >

    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[command(about = "File processor")]
    struct Cli {
        /// Input file (positional argument)
        input: PathBuf,

        /// Output file (option: --output or -o)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Print extra info (flag: --verbose or -v)
        #[arg(short, long)]
        verbose: bool,
    }

    fn main() {
        let args = Cli::parse();

        if args.verbose {
            println!("Input: {:?}", args.input);
        }

        match args.output {
            Some(path) => println!("Writing to {:?}", path),
            None       => println!("Writing to stdout"),
        }
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">cargo run -- input.txt
-v --output out.txt</span>  
<span class="out">Input: "input.txt"</span>  
<span class="out">Writing to "out.txt"</span> ** >



> [!TIP] > **short vs long** >

`#[arg(short)]` = use first letter of field name as `-x` flag.
`#[arg(long)]` = use full field name as `--field-name`. You can
override: `#[arg(short = 'O', long = "out")]`.

** >