---
title: "Subcommands"
description: ""
---

subcommands">

<div class="section-eyebrow">

Chapter 9** >

## Subcommands

Subcommands let you build CLIs like `git` where the first argument picks
an operation (`git add`, `git commit`, `git push`). In clap, subcommands
are **enum variants**.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>
<span class="code-filename">src/main.rs</span>

copy

** >

    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(about = "A task manager")]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Add a new task
        Add {
            /// Task description
            description: String,
            /// Priority level
            #[arg(short, long, default_value_t = 1)]
            priority: u8,
        },
        /// List all tasks
        List {
            /// Show only done tasks
            #[arg(long)]
            done: bool,
        },
        /// Remove a task by ID
        Remove {
            id: u64,
        },
    }

    fn main() {
        let cli = Cli::parse();

        match cli.command {
            Commands::Add { description, priority } => {
                println!("Adding task: '{}' (priority {})", description, priority);
            }
            Commands::List { done } => {
                println!("Listing {} tasks", if done { "done" } else { "all" });
            }
            Commands::Remove { id } => {
                println!("Removing task {}", id);
            }
        }
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./tasks add "buy milk"
--priority 2</span>  
<span class="out">Adding task: 'buy milk' (priority 2)</span>  
  
<span class="prompt">\$</span> <span class="cmd">./tasks list
--done</span>  
<span class="out">Listing done tasks</span>  
  
<span class="prompt">\$</span> <span class="cmd">./tasks --help</span>  
<span class="out">A task manager</span>  
<span class="out">Usage: tasks <COMMAND></span>  
<span class="out">Commands:</span>  
<span class="out"> add Add a new task</span>  
<span class="out"> list List all tasks</span>  
<span class="out"> remove Remove a task by ID</span> ** >