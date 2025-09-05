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
| `flurry::HashMap` | 320.87 | 80.86 | 37.16 | 18.91 | 8.01 | 5.42 | 2.61 |
| `hashbrown::HashMap` | 21.30 | 10.73 | 5.90 | 2.78 | 1.10 | 0.79 | 0.30 |
| `heapless::LinearMap` | 1.43 | 1.35 | 1.09 | 1.05 | 1.05 | 1.16 | 1.02 |
| `indexmap::IndexMap` | 12.55 | 10.74 | 6.89 | 3.58 | 1.46 | 1.00 | 0.48 |
| `linear_map::LinearMap` | 1.59 | 1.39 | 0.98 | 0.78 | 0.64 | 1.00 | 1.09 |
| `linked_hash_map::LinkedHashMap` | 27.06 | 19.09 | 11.73 | 5.72 | 2.31 | 1.61 | 0.77 |
| `litemap::LiteMap` | 1.62 | 2.21 | 4.15 | 3.52 | 1.29 | 1.03 | 0.61 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.42 | 10.92 | 6.47 | 2.65 | 1.03 | 0.71 | 0.37 |
| `rustc_hash::FxHashMap` | 20.72 | 10.83 | 5.94 | 2.62 | 0.93 | 0.65 | 0.32 |
| `std::collections::BTreeMap` | 21.08 | 14.21 | 9.18 | 5.32 | 1.97 | 1.40 | 0.78 |
| `std::collections::HashMap` | 21.26 | 13.14 | 7.68 | 3.90 | 1.73 | 1.13 | 0.55 |
| `tinymap::array_map::ArrayMap` | 1.97 | 3.89 | 3.84 | 3.66 | 3.07 | 4.86 | 4.54 |

The experiment [was performed][action] on 05-09-2025.
There were 1000000 repetitions.
The entire benchmark took 274s.
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
