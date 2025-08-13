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

First, add this to `Cargo.toml`:

```toml
[dependencies]
micromap = "0.1.0"
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
| `flurry::HashMap` | 305.69 | 74.31 | 39.74 | 14.52 | 9.11 | 5.03 | 2.82 |
| `hashbrown::HashMap` | 20.79 | 9.95 | 6.34 | 2.29 | 1.24 | 0.74 | 0.37 |
| `heapless::LinearMap` | 1.18 | 1.30 | 1.18 | 0.92 | 0.87 | 1.22 | 1.02 |
| `indexmap::IndexMap` | 16.08 | 13.13 | 8.80 | 3.58 | 2.25 | 1.19 | 0.70 |
| `linear_map::LinearMap` | 1.77 | 1.34 | 0.99 | 0.63 | 0.76 | 1.11 | 1.08 |
| `linked_hash_map::LinkedHashMap` | 26.99 | 18.48 | 11.40 | 4.58 | 2.75 | 1.44 | 0.85 |
| `litemap::LiteMap` | 1.59 | 1.96 | 4.00 | 2.77 | 1.51 | 0.92 | 0.64 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.54 | 10.24 | 6.81 | 2.02 | 1.30 | 0.70 | 0.40 |
| `rustc_hash::FxHashMap` | 20.78 | 10.14 | 6.27 | 2.15 | 1.10 | 0.59 | 0.32 |
| `std::collections::BTreeMap` | 19.95 | 8.05 | 4.88 | 2.75 | 1.89 | 1.05 | 0.75 |
| `std::collections::HashMap` | 20.51 | 12.50 | 7.91 | 3.16 | 2.01 | 1.04 | 0.59 |
| `tinymap::array_map::ArrayMap` | 1.98 | 3.77 | 3.89 | 2.94 | 4.06 | 4.43 | 4.92 |

The experiment [was performed][action] on 13-08-2025.
There were 1000000 repetitions.
The entire benchmark took 269s.
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

* Linux:

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps
```

* Windows(PowerShell):

```PowerShell
$env:RUSTDOCFLAGS="--cfg docsrs"; cargo +nightly doc --all-features --no-deps --open; Remove-Item Env:\RUSTDOCFLAGS
```

Then, after the changes you make, run it again.
Compare the results.
If your changes degrade the performance,
think twice before submitting a pull request.

About the **version change**, we follow the rules of this
[Cargo SemVer reference](https://doc.rust-lang.org/cargo/reference/semver.html)
. If your code has an impact on semver compatibility, such as
**breaking changes**, then you may also need to explicitly upgrade the version.
Because our project version uses a placeholder, you can
_add a hint note after the version number `0.0.0`_ in Cargo.toml
`package.version` to mark that you want to update the version, which we call
"version hint", as follows:

```toml
[package]
name = "micromap"
version = "0.0.0" # hint: 1.2.3
# ...
```

If no version change is required, do not add any comments after the version
number `0.0.0`.

[std]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[rs]: https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs
[action]: https://github.com/yegor256/micromap/actions/workflows/benchmark.yml
