# The Fastest Linear Map in Rust

[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)

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
micromap = "0.0.15"
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
is designed as closely similar to
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
| `flurry::HashMap` | 250.37 | 68.06 | 29.90 | 13.14 | 6.38 | 3.66 | 2.10 |
| `hashbrown::HashMap` | 20.69 | 12.10 | 6.72 | 2.39 | 1.25 | 0.64 | 0.30 |
| `heapless::LinearMap` | 1.22 | 1.58 | 1.23 | 0.92 | 1.04 | 0.97 | 1.18 |
| `indexmap::IndexMap` | 12.98 | 13.04 | 7.74 | 3.17 | 1.67 | 0.94 | 0.49 |
| `linear_map::LinearMap` | 1.67 | 1.69 | 1.06 | 0.74 | 0.72 | 0.94 | 1.00 |
| `linked_hash_map::LinkedHashMap` | 26.42 | 22.70 | 12.65 | 5.18 | 2.72 | 1.57 | 0.80 |
| `litemap::LiteMap` | 1.69 | 2.89 | 5.40 | 2.71 | 1.74 | 0.93 | 0.60 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.65 | 12.51 | 7.23 | 2.25 | 1.24 | 0.68 | 0.33 |
| `rustc_hash::FxHashMap` | 20.43 | 12.25 | 7.01 | 2.75 | 0.98 | 0.55 | 0.32 |
| `std::collections::BTreeMap` | 19.88 | 9.99 | 5.38 | 3.13 | 1.81 | 1.16 | 0.71 |
| `std::collections::HashMap` | 20.49 | 15.88 | 8.91 | 3.57 | 1.92 | 1.07 | 0.56 |
| `tinymap::array_map::ArrayMap` | 1.99 | 4.64 | 4.34 | 3.10 | 3.48 | 4.30 | 4.36 |

The experiment [was performed][action] on 20-03-2025.
There were 1000000 repetition cycles.
The entire benchmark took 241s.
Uname: 'Linux'.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that
were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

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
rustup run nightly cargo bench
```

Then, after the changes you make, run it again. Compare the results.
If your changes
degrade performance, think twice before submitting a pull request.

[std]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[rs]: https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs
[action]: https://github.com/yegor256/micromap/actions/workflows/benchmark.yml
