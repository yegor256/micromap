# Fuzz Testing for Issue #299

This directory hosts the fuzzing infrastructure that closes [Issue #299](https://github.com/yegor256/micromap/issues/299): *"we don''t use fuzz testing"*. The goal is to continuously stress the `micromap::Map` API against random sequences of operations and keep parity with a reference `HashMap` implementation.

## Layout

- `Cargo.toml` — standalone fuzz crate (`micromap-fuzz`).
- `src/` — shared support code:
  - `input.rs` defines the `Op` enum, probabilities, and bounds (`MAX_OPS = 64`).
  - `apply.rs` executes operations on both `micromap::Map` and the shadow `HashMap`, using `checked_insert` to avoid aborts.
- `fuzz_targets/fuzz_map_basic.rs` — main fuzz target wired into libFuzzer.
- `corpus/` (auto-created) — initial inputs; empty by default.
- `artifacts/` — crashes, timeouts, hangs produced by the fuzzer.
- `fuzz-findings.md` — running log of investigations (see also `tests/regressions/README.md`).

## Prerequisites

- Rust stable toolchain (for the library itself).
- Rust nightly toolchain with minimal profile:
  ```bash
  rustup toolchain install nightly --profile minimal
  ```
- `cargo-fuzz` binary:
  ```bash
  cargo install cargo-fuzz
  ```

On Windows the recommended approach is to run fuzzing under WSL/Ubuntu to avoid missing `clang_rt.fuzzer` DLLs.

## Quick Verification Steps

Run from the repository root (`~/micromap`):

```bash
cargo fmt --all
cargo check
cargo test
cargo check --manifest-path fuzz/Cargo.toml
cargo +nightly fuzz run fuzz_map_basic -- -max_total_time=60
```

The final command performs a 60-second fuzz campaign using libFuzzer. Adjust the duration via `-max_total_time=<seconds>`.

## Interpreting Results

- Crashes are stored under `fuzz/artifacts/fuzz_map_basic/` with a `crash-*` filename.
- Reproduce a crash locally:
  ```bash
  cargo fuzz run fuzz_map_basic fuzz/artifacts/fuzz_map_basic/<crash-file>
  ```
- Minimise an input before turning it into a regression test:
  ```bash
  cargo fuzz tmin fuzz_map_basic fuzz/artifacts/fuzz_map_basic/<crash-file>
  ```
- Add a deterministic regression test to `tests/regressions/` using the `template.rs` scaffold and document it in `fuzz/fuzz-findings.md`.

The `apply_op` helper keeps the shadow `HashMap` aligned even when the map reaches capacity, so panics should only indicate genuine bugs.

## CI Integration

Two GitHub Actions workflows exercise the fuzz target:

- `.github/workflows/ci-fuzz.yml` — short (~60s) sanity check on each PR.
- `.github/workflows/nightly-fuzz.yml` — hourly fuzzing with sanitizers and artifact upload on failure.

## Suggested Enhancements

- Save the recommended dictionary produced by `cargo fuzz` into `fuzz/dictionaries/` to accelerate future campaigns.
- Track outstanding findings in `fuzz/fuzz-findings.md` and mirror resolved ones in regression tests.

## Useful Commands

```bash
# Run with a persistent corpus directory
cargo +nightly fuzz run fuzz_map_basic -- -max_total_time=300

# Reset corpus/artifacts
rm -rf fuzz/corpus/fuzz_map_basic fuzz/artifacts/fuzz_map_basic

# Switch to a different fuzzing mutator budget
cargo +nightly fuzz run fuzz_map_basic -- -runs=100000
```

Maintain this README alongside the fuzz crate whenever the API or workflows evolve.
