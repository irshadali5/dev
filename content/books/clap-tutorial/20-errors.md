---
title: "Error Handling"
description: ""
---

errors">

<div class="section-eyebrow">

Chapter 21** >

## Error Handling

`Cli::parse()` exits on error. If you want to handle errors yourself
(e.g. in tests, or for custom error formatting), use `Cli::try_parse()`.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    fn main() {
        let cli = Cli::try_parse().unwrap_or_else(|e| {
            // e.kind() lets you check the error type
            eprintln!("Error: {e}");
            std::process::exit(1);
        });

        // Or use the built-in pretty exit:
        let cli = Cli::try_parse().unwrap_or_else(|e| e.exit());
    }

** >

### Testing your CLI

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[test]
    fn test_defaults() {
        let cli = Cli::parse_from(["myapp"]);  // inject args
        assert_eq!(cli.port, 8080);
    }

    #[test]
    fn test_invalid_port() {
        let result = Cli::try_parse_from(["myapp", "--port", "99999"]);
        assert!(result.is_err());
    }

** >



> [!TIP] > **Always test your CLI** >

Use `parse_from` and `try_parse_from` in unit tests. They accept
anything that iterates over strings — no need for integration tests just
to check argument parsing.

** >