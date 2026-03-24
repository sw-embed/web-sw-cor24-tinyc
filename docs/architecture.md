# web-tc24r Architecture

## Overview

web-tc24r is a browser-based UI for the tc24r C compiler targeting the COR24 ISA.
Users write COR24 assembly (and eventually C code), compile, and run it entirely
in the browser via WebAssembly.

## Component Pipeline

```
User Input (browser)
       │
       ▼
┌─────────────┐     ┌──────────────┐     ┌──────────────┐
│  Yew App    │────▶│  tc24r WASM  │────▶│ cor24-rs WASM│
│  (UI layer) │     │  (C compiler)│     │ (asm + emu)  │
└─────────────┘     └──────────────┘     └──────────────┘
       │                    │                    │
       │              C → COR24 asm        asm → machine
       │                                   code → execute
       ▼
   Output display
   (assembly, registers, UART output)
```

## How tc24r Will Be Embedded as WASM

tc24r is a pure Rust library with zero external dependencies. Its compilation
pipeline (preprocess → lex → parse → codegen) operates entirely on in-memory
strings, making it ideal for WASM.

### Integration Plan

1. **Add tc24r crates as path dependencies** in `Cargo.toml`, pointing to
   `~/github/sw-vibe-coding/tc24r/components/*/crates/*`.

2. **Create a thin WASM bridge module** (`src/compiler.rs`) that calls:
   ```rust
   let preprocessed = tc24r_preprocess::preprocess(source, None, &[]);
   let tokens = tc24r_lexer::Lexer::new(&preprocessed).tokenize()?;
   let program = tc24r_parser::parse(tokens)?;
   let mut codegen = tc24r_codegen::Codegen::new();
   Ok(codegen.generate(&program))
   ```

3. **Add cor24-rs as a path dependency** pointing to
   `~/github/sw-embed/cor24-rs`. cor24-rs already has WASM bindings (`WasmCpu`)
   that provide `assemble()`, `step()`, `run_batch()`, and state inspection.

4. **Wire UI to pipeline**: the Run button triggers compilation, feeds the
   resulting assembly to cor24-rs's assembler, and runs the emulator. Output
   (registers, memory, UART) is displayed in the output pane.

### Key Crates from tc24r

| Crate              | Role                              |
|--------------------|-----------------------------------|
| `tc24r-preprocess` | `#define`, `#include`, `#pragma`   |
| `tc24r-lexer`      | C source → token stream           |
| `tc24r-parser`     | Token stream → AST                |
| `tc24r-codegen`    | AST → COR24 assembly text         |
| `tc24r-ast`        | Shared AST types                  |
| `tc24r-token`      | Shared token types                |
| `tc24r-error`      | Compiler error types              |

### Key Crate from cor24-rs

| Crate       | Role                                          |
|-------------|-----------------------------------------------|
| `cor24-rs`  | COR24 assembler + emulator with WASM bindings |

## Technology Stack

- **Rust + Yew 0.21** — component framework (CSR mode)
- **Trunk** — WASM build tool and dev server
- **wasm-bindgen** — Rust ↔ JS interop
- **Edition 2024** — latest Rust edition

## COR24 Architecture Summary

- 24-bit RISC ISA, 8 registers (r0-r2 GP, fp, sp, z, iv, ir)
- 32 instructions, variable-length encoding (1/2/4 bytes)
- 24-bit address space (16 MB), memory-mapped I/O
- UART at `0xFF0100`, LED at `0xFF0000`
