[![cargo](https://github.com/objectionary/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/objectionary/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![PDD status](http://www.0pdd.com/svg?name=objectionary/micromap)](http://www.0pdd.com/p?name=objectionary/micromap)
[![codecov](https://codecov.io/gh/objectionary/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/objectionary/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/objectionary/micromap)](https://hitsofcode.com/view/github/objectionary/micromap)
![Lines of code](https://img.shields.io/tokei/lines/github/objectionary/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/objectionary/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)

A faster alternative to Rust `HashMap`, for very small maps.

```rust
use micromap::MicroMap;
let mut m : MicroMap<u64, &str, 10> = MicroMap::new();
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Read [the documentation](https://docs.rs/micromap/latest/micromap/).

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
$ cargo test -vv
```

If everything goes well, fork repository, make changes, send us a [pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also, 
run `cargo fmt` and `cargo clippy`.
