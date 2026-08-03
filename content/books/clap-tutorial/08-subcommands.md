---
title: "Subcommands"
description: ""
---




## Subcommands

Subcommands let you build CLIs like `git` where the first argument picks
an operation (`git add`, `git commit`, `git push`). In clap, subcommands
are **enum variants**.



```rust

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

```


\$ ./tasks add "buy milk"
--priority 2  
Adding task: 'buy milk' (priority 2)  
  
\$ ./tasks list
--done  
Listing done tasks  
  
\$ ./tasks --help  
A task manager  
Usage: tasks <COMMAND>  
Commands:  
 add Add a new task  
 list List all tasks  
 remove Remove a task by ID 
