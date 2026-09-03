# Fuzz Testing for micromap

This directory contains fuzz tests for the micromap crate using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

## Prerequisites

1. Install nightly Rust toolchain:
   ```bash
   rustup toolchain install nightly
   ```

2. Install cargo-fuzz:
   ```bash
   cargo install cargo-fuzz
   ```

## Running Fuzz Tests

To run the fuzz tests, use the nightly toolchain:

### Basic Map Operations
Test fundamental Map operations like insert, get, remove, clear, and retain:
```bash
cargo +nightly fuzz run map_operations
```

### Set Operations  
Test Set operations like insert, contains, remove, clear, and set-specific operations:
```bash
cargo +nightly fuzz run set_operations
```

### Edge Cases
Test complex scenarios, different capacities, and the Entry API:
```bash
cargo +nightly fuzz run edge_cases
```

## Fuzz Test Targets

- **map_operations**: Tests basic Map operations with `u8` keys and values on a map with capacity 16
- **set_operations**: Tests basic Set operations with `u8` values on a set with capacity 16, including set-specific operations like `is_disjoint`, `is_subset`, `is_superset`
- **edge_cases**: Tests complex scenarios with different map capacities (1, 4, 8, 32), `u16` keys/values, Entry API, and stress-tests iteration consistency

## Continuous Fuzzing

For longer fuzzing sessions, you can specify a time limit or number of runs:

```bash
# Run for 60 seconds
cargo +nightly fuzz run map_operations -- -max_total_time=60

# Run for specific number of iterations
cargo +nightly fuzz run set_operations -- -runs=10000
```

## Corpus and Artifacts

- **Corpus**: Interesting inputs found during fuzzing are stored in `corpus/`
- **Artifacts**: Crashing inputs are stored in `artifacts/`

To reproduce a crash:
```bash
cargo +nightly fuzz run <target> artifacts/<target>/crash-<hash>
```

To minimize a crashing input:
```bash
cargo +nightly fuzz tmin <target> artifacts/<target>/crash-<hash>
```

## Coverage

To generate coverage information:
```bash
cargo +nightly fuzz coverage <target>
```

This will run the target against the collected corpus and generate coverage reports.