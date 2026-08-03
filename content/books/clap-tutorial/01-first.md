---
title: "Your First clap Program"
description: ""
---




## Your First clap Program

The derive approach: you define a **struct** and annotate it with
`#[derive(Parser)]`. clap reads those annotations and builds the entire
parser from your struct's shape.



```rust

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

```


\$ cargo run --
irshad  
Hello, irshad!  
  
\$ cargo run --
--help  
A greeting CLI  
  
Usage: greet <NAME>  
  
Arguments:  
 <NAME> Name of the person to greet  
  
Options:  
 -h, --help Print help  
 -V, --version Print version 



> [!TIP] > **Key insight

The `/// doc comment` on each field becomes the help text automatically.
This is not magic — clap's proc macro reads it at compile time. Always
write doc comments on your fields.


### What just happened?

Three things the macro wired up for you:

| You wrote | clap did |
|----|----|
| `struct Cli { name: String }` | Registered a positional argument called NAME |
| `#[command(about = "...")]` | Set the help description line |
| `Cli::parse()` | Parsed argv, validated types, printed errors & exited on failure |
