# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

This project uses AgentRail. Every session follows this exact sequence:

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It tells you your current step, prompt, skill docs, and past trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask the user "want me to proceed?" or "shall I start?". The step prompt IS your instruction. Execute it.

### 4. COMMIT (after the work is done)
Commit your code changes with git.

### 5. COMPLETE (LAST thing, after committing)
```bash
agentrail complete --summary "what you accomplished" \\
  --reward 1 \\
  --actions "tools and approach used"
```
If the step failed: `--reward -1 --failure-mode "what went wrong"`
If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make any further code changes after running agentrail complete.
Any changes after complete are untracked and invisible to the next session.
If you see more work to do, it belongs in the NEXT step, not this session.

Do NOT skip any of these steps. The next session depends on your trajectory recording.

## Project

Web UI for the tc24r compiler. Live browser demos of COR24 compilation using Rust, Yew, and WebAssembly.

## Related Projects

- `~/github/sw-vibe-coding/tc24r` -- The COR24 compiler (Rust) -- this is what we embed as WASM
- `~/github/sw-embed/cor24-rs` -- COR24 assembler and emulator (Rust)
- `~/github/sw-vibe-coding/tml24c` -- Tiny Macro Lisp for COR24 (C)
- `~/github/sw-vibe-coding/agentrail-domain-coding` -- Coding skills domain

## Available Task Types

`rust-project-init`, `rust-clippy-fix`, `rust-test-write`, `yew-component`, `wasm-build`, `pre-commit`

## Build

```bash
trunk serve    # dev server with hot reload
trunk build    # production build
```

Edition 2024. Clippy: `cargo clippy -- -D warnings`. Never suppress warnings.
