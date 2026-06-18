# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

A catalog of 26 Rust↔C FFI safety hazards. Each hazard has a documented description
(`EvaluationCatalog.md`), a C implementation with a *legal* and an *illegal* variant
(`evaluation_catalog/NN_name.c`), an `extern "C"` declaration
(`ffi_safty_evaluator/src/ffi.rs`), and a Rust unit test that exercises both variants
(`ffi_safty_evaluator/src/lib.rs`). The "illegal" variants are deliberately unsound
under an assumed Rust-side binding — the point is to test Rust-side FFI assumptions, not
whether the C compiler accepts the code.

## Commands

Everything builds and runs through the Cargo crate; `build.rs` compiles the C sources
via the `cc` crate automatically — you do **not** need to run `make` first.

```bash
cd ffi_safty_evaluator
cargo test                       # build C catalog + run all 26×2 FFI tests
cargo test fill_bool_illegal     # run a single test by name
cargo build                      # compile only
```

The standalone Makefile in `evaluation_catalog/` (`make`, `make clean`) builds
`libeval_catalog.a` independently of Cargo; it exists for inspecting the C objects in
isolation and is not part of the normal Rust workflow.

## Architecture and the "four files per case" invariant

The catalog is numbered 1–26 and every case must stay consistent across four places:

1. `EvaluationCatalog.md` — prose: description, *assumed Rust-side binding*, legal code,
   illegal code, and why it's unsound. This is the source of truth for intent.
2. `evaluation_catalog/NN_name.c` — the C implementation, exporting a `*_legal` and a
   `*_illegal` symbol.
3. `ffi_safty_evaluator/src/ffi.rs` — one `extern "C"` declaration per symbol, grouped
   under a `// --- Case NN: ... ---` comment. Shared aggregate types (`ByteSlice`,
   `Pair`/`PairPacked`, `Header`/`HeaderReversed`) carry the `repr` that matches the
   C layout — these reprs are part of the test (e.g. `#[repr(C, packed)]` for case 15).
4. `ffi_safty_evaluator/src/lib.rs` — `#[cfg(test)] mod tests`, two tests per case
   (`*_legal` / `*_illegal`).

When adding or changing a case, update all four in lockstep, and register any new `.c`
file in the `cc::Build::files([...])` list in `build.rs`.

## C-side gotchas baked into the build

- **Case 20** (`20_concurrent_mutation.c`) uses pthreads; `build.rs` emits
  `cargo:rustc-link-lib=pthread`. Any new pthread-using case relies on this already
  being linked.
- **Cases 07 and 24** contain intentional undefined behavior (returning a stack address;
  reading an uninitialized scalar). Their compiler warnings are suppressed with
  `#pragma GCC diagnostic` inside the source files — do not "fix" these; the UB is the
  test.
- Tests assert the *observed* (often unsound) behavior of the illegal variants, so they
  are inherently sensitive to compiler/optimization choices. Treat surprising failures
  on a new toolchain as expected fragility, not necessarily a regression.

## Conventions

- Edition 2024; the only dependency is `cc` (build-time).
- C uses `-Wall -Wextra -std=c11`, K&R-style function definitions (return type on its own
  line), and `stdint.h` fixed-width types.
- Committed `.o` / `.a` artifacts under `evaluation_catalog/` predate the `.gitignore`
  rules that now exclude them; leave them unless asked to clean up.
