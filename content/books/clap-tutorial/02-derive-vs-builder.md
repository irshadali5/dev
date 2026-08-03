---
title: "Derive API vs Builder API"
description: ""
---

derive-vs-builder">

<div class="section-eyebrow">

Chapter 3** >

## Derive API vs Builder API

clap has two APIs. **Always use Derive** unless you have a compelling
reason not to.

<div class="compare">

<div class="compare-col">

<div class="compare-label derive">

✓ Derive API (recommended)** >

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>** >

    #[derive(Parser)]
    struct Cli {
      /// Port number
      #[arg(short, long, default_value_t = 8080)]
      port: u16,
    }

** > ** >

<div class="compare-col">

<div class="compare-label builder">

Builder API (verbose)** >

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>** >

    Command::new("app")
      .arg(Arg::new("port")
        .short('p')
        .long("port")
        .default_value("8080")
        .value_parser(
          clap_value_parser!(u16)))

** > ** > ** >

The Builder API gives you more runtime flexibility (dynamic commands)
but most CLIs don't need that. Stick with Derive.