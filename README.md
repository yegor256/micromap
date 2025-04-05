# The Fastest Linear Map in Rust

[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)
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
| `flurry::HashMap` | 265.09 | 87.20 | 42.94 | 21.88 | 11.91 | 5.37 | 2.43 |
| `hashbrown::HashMap` | 20.67 | 10.76 | 6.80 | 3.33 | 1.54 | 0.63 | 0.26 |
| `heapless::LinearMap` | 1.09 | 1.41 | 1.20 | 1.21 | 1.04 | 1.12 | 0.85 |
| `indexmap::IndexMap` | 13.34 | 11.88 | 7.63 | 5.80 | 2.14 | 1.01 | 0.43 |
| `linear_map::LinearMap` | 1.65 | 1.48 | 1.07 | 1.13 | 1.02 | 1.24 | 1.03 |
| `linked_hash_map::LinkedHashMap` | 26.22 | 19.95 | 12.33 | 6.91 | 3.39 | 1.58 | 0.72 |
| `litemap::LiteMap` | 1.64 | 2.20 | 4.62 | 3.32 | 2.23 | 1.02 | 0.52 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.54 | 11.10 | 7.41 | 3.03 | 1.53 | 0.68 | 0.32 |
| `rustc_hash::FxHashMap` | 20.38 | 10.84 | 6.89 | 3.07 | 1.28 | 0.64 | 0.28 |
| `std::collections::BTreeMap` | 25.05 | 10.34 | 5.44 | 4.31 | 2.47 | 1.18 | 0.62 |
| `std::collections::HashMap` | 20.70 | 14.14 | 8.63 | 4.89 | 2.60 | 1.23 | 0.51 |
| `tinymap::array_map::ArrayMap` | 1.63 | 4.38 | 4.57 | 4.93 | 4.32 | 4.58 | 3.88 |

The experiment [was performed][action] on 31-03-2025.
There were 1000000 repetition cycles.
The entire benchmark took 242s.
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
