---
title: "Toolchain & Setup"
description: "Use rustup — the official Rust toolchain manager:"
order: 1
---

## Installing Rust 1.96.0

Use `rustup` — the official Rust toolchain manager:

**`terminal`**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
# rustc 1.96.0 (release 2026-05-15)

cargo --version
# cargo 1.96.0
```

## Essential Tools

**`terminal`**

```bash
# SQLx CLI for migrations
cargo install sqlx-cli --no-default-features --features postgres

# Cargo watch for inner development loop
cargo install cargo-watch

# Cargo expand for macro debugging
cargo install cargo-expand

# Nextest — faster test runner
cargo install cargo-nextest

# Cargo deny for dependency auditing
cargo install cargo-deny

# Bunyan CLI for structured log viewing
cargo install bunyan
```

## Faster Linking

Rust compilation can be slow. Use faster linkers:

**`.cargo/config.toml`**

```toml
# Linux — use mold (fastest linker in 2026)
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

# macOS — zld
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/bin/zld"]

# Windows — lld
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

## IDE Setup

Editor

Plugin

Notes

VS Code

rust-analyzer

Best free option

Zed

Built-in

Native Rust support, fastest

Helix

Built-in LSP

Terminal-native, zero config

JetBrains

RustRover 2026.1

Best paid option

  
> [!IMPORTANT]  
> For true production readiness, cargo build isn't enough. We must guarantee reproducible builds. By utilizing Nix Flakes, we can pin the exact versions of system dependencies (like OpenSSL or \`libc\`) alongside our Rust toolchain. **`flake.nix`** inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; rust-overlay.url = "github:oxalica/rust-overlay"; }; Furthermore, we cross-compile to x86_64-unknown-linux-musl to produce fully static binaries. This allows us to use Distroless Docker images (or even \`scratch\`), eliminating the attack surface of a traditional OS environment like Debian or Alpine.