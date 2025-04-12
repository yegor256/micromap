# The Fastest Linear Map in Rust

[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)
[![MSRV](https://img.shields.io/badge/MSRV-1.79-ffc832)](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fyegor256%2Fmicromap.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fyegor256%2Fmicromap?ref=badge_shield&issueType=license)

A much faster alternative of
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
for very small maps.
It is also faster than
[FxHashMap](https://github.com/rust-lang/rustc-hash),
[hashbrown](https://github.com/rust-lang/hashbrown),
[ArrayMap](https://github.com/robjtede/tinymap),
[IndexMap](https://crates.io/crates/indexmap),
and _all_ others.
The smaller the map, the higher the performance.
It was observed that when a map contains more than 20 keys,
it may be better to use the standard
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
since the performance of `micromap::Map` _may_ start to degrade.
See the [benchmarking results](#benchmark) below.

**WELCOME**:
Not all functions that you might expect to have in a map are implemented.
I will appreciate if you contribute by implementing these
[missing functions](https://github.com/yegor256/micromap/issues).

First, add this to `Cargo.toml`:

```toml
[dependencies]
micromap = "0.0.17"
```

Then, use it like a standard hash map... well, almost:

```rust
use micromap::Map;
let mut m : Map<u64, &str, 10> = Map::new(); // allocation on stack
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`.
This is the total size of the map, which is allocated on stack when `::new()`
is called. Unlike `HashMap`, the `Map` doesn't use heap at all. If more than
ten keys will be added to the map, it will panic.

Read [the API documentation](https://docs.rs/micromap/latest/micromap/).
The struct
[`micromap::Map`](https://docs.rs/micromap/latest/micromap/struct.Map.html)
is designed to be as closely similar to
[`std::collections::HashMap`][std] as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `micromap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions
([`benchmark.rs`][rs])
to them and measured how fast they performed. In the following table,
the numbers over 1.0 indicate performance gain,
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 2 | 4 | 8 | 16 | 32 | 64 | 128 |
| --- | --: | --: | --: | --: | --: | --: | --: |
| `flurry::HashMap` | 283.44 | 84.25 | 40.99 | 22.93 | 11.46 | 5.17 | 2.50 |
| `hashbrown::HashMap` | 20.36 | 10.78 | 6.68 | 3.24 | 1.48 | 0.65 | 0.30 |
| `heapless::LinearMap` | 1.08 | 1.41 | 1.20 | 1.23 | 1.04 | 1.05 | 0.86 |
| `indexmap::IndexMap` | 13.11 | 11.53 | 7.49 | 5.94 | 2.18 | 0.94 | 0.44 |
| `linear_map::LinearMap` | 1.62 | 1.49 | 1.06 | 1.10 | 1.01 | 1.15 | 1.02 |
| `linked_hash_map::LinkedHashMap` | 26.22 | 20.17 | 12.37 | 6.73 | 3.46 | 1.59 | 0.70 |
| `litemap::LiteMap` | 1.62 | 2.19 | 4.35 | 3.18 | 2.23 | 0.96 | 0.52 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.38 | 11.10 | 7.37 | 2.97 | 1.50 | 0.65 | 0.32 |
| `rustc_hash::FxHashMap` | 20.07 | 10.80 | 6.85 | 2.95 | 1.28 | 0.61 | 0.28 |
| `std::collections::BTreeMap` | 24.77 | 10.41 | 5.48 | 4.30 | 2.44 | 1.13 | 0.63 |
| `std::collections::HashMap` | 20.71 | 13.81 | 8.63 | 4.70 | 2.64 | 1.07 | 0.51 |
| `tinymap::array_map::ArrayMap` | 1.98 | 4.39 | 4.54 | 4.83 | 4.34 | 4.34 | 3.91 |

The experiment [was performed][action] on 07-04-2025.
There were 1000000 repetition cycles.
The entire benchmark took 248s.
Uname: 'Linux'.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that
were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## MSRV (Minimum Supported Rust Version)

**`Rust 1.79`**

(Enabling some features will affect MSRV, the documentation will note it.)

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install), update to the
last version by `rustup update stable`, and then:

```bash
cargo test -vv
```

If everything goes well, fork repository, make changes, send us a
[pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also,
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
cargo bench --bench bench
```

If you modified the comment docs, run this to check:

- Linux:

    ```bash
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps
    ```

- Windows(PowerShell):

    ```PowerShell
    $env:RUSTDOCFLAGS="--cfg docsrs"; cargo +nightly doc --all-features --no-deps --open; Remove-Item Env:\RUSTDOCFLAGS
    ```

Then, after the changes you make, run it again. Compare the results.
If your changes
degrade performance, think twice before submitting a pull request.

[std]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[rs]: https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs
[action]: https://github.com/yegor256/micromap/actions/workflows/benchmark.yml
