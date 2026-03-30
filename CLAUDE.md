# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Web UI for the sw-cor24-tinyc Tiny C compiler. Live browser demos of COR24 compilation using Rust, Yew, and WebAssembly.

## Related Projects

- `~/github/sw-embed/sw-cor24-tinyc` — Tiny C compiler for COR24 (Rust) — embedded as WASM
- `~/github/sw-embed/sw-cor24-emulator` — COR24 assembler and emulator (Rust)
- `~/github/sw-embed/sw-cor24-project` — COR24 ecosystem hub

## Build

```bash
./scripts/serve.sh              # dev server with hot reload on port 9101
./scripts/build-pages.sh        # production build to pages/
trunk build                     # basic production build
```

Edition 2024. Clippy: `cargo clippy -- -D warnings`. Never suppress warnings.
