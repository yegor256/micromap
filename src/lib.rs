// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! ## [`Map`]
//! This is a **simpler and faster** alternative implementation of the standard
//! [`HashMap`][std::collections::HashMap] in **small-scale**. It does **not use
//! heap** and **not need the [`Hash`][core::hash::Hash] or [`Ord`] trait** at all.
//!
//! It simply keeps all key-value pairs in an array and when it's necessary to
//! retrieve by key, it scrolls through the entire array. This implementation
//! works much faster for small maps of **less than 50 keys (recommended)**, but
//! definitely is not suitable for larger maps.
//!
//! Check [this page](https://github.com/yegor256/micromap#benchmark)
//! for the recent benchmarking results.
//!
//! For example, here is how a map with a few keys can be created:
//! ```
//! use micromap::Map;
//! let mut map : Map<u64, &str, 10> = Map::new();
//! map.insert(1, "Hello, world!");
//! map.insert(2, "Good bye!");
//! assert_eq!(map.len(), 2);
//! assert_eq!(map.capacity(), 10);
//! ```
//!
//! Creating a [`Map`] requires knowing the maximum size of it, upfront. This is
//! what the third type argument `10` is for, in the example above. The array
//! will have exactly ten elements. An attempt to [`insert`][Map::insert] an 11th
//! element will lead to a panic. (Or use [`checked_insert`][Map::checked_insert]
//! instead to avoid panics by returning an [`Option`].)
//!
//! ## [`Set`]
//! Similarly, you can also create a **small [`Set`] on stack** directly. It has basic
//! but adequate operations as a real set. (As is common practice, `Set<T>` is actually
//! a `Map<T, ()>`)
//!
//! For example, here is how a set with a few items can be created and manipulated:
//! ```
//! use micromap::Set;
//! let v = vec![5, 1, 2, 4];
//! let set_a = Set::from([1, 2, 3]); // Set<i32, 3>
//! let set_b: Set<i32, 4> = Set::from_iter(v);
//! assert_eq!(set_a.capacity(), 3);
//! assert_eq!(set_b.capacity(), set_a.capacity() + 1);
//! // 5 or more (e.g. 7), or else panic
//! let set_union: Set<_, 7> = set_a.union(&set_b).cloned().collect();
//! let set_diff = &set_a - &set_b;
//! assert_eq!(set_union, Set::from([1, 2, 3, 4, 5]));
//! assert_eq!(set_diff, Set::from([3]));
//! ```
//! ## `no_std` support and Optional features
//! The `micromap` does not depend on [`std`], so `no_std` is supported by default.
//!
//! And no feature is enabled by default:
//! - `serde`: When this optional dependency is enabled, micromap implements the
//!   `serde::Serialize` and `serde::Deserialize` traits.
//! - `std`: Currently no effect, you can just ignore it.
//!
//! ## Capacity and Allocation
//! The capacity **cannot be changed after creation** unless a new instance is created
//! using a different generic constant. **No heap allocations** are made.
//!
//! If the type of the `Map` key-value pair (or the item type of the `Set`) is large, a
//! stack-overflow error may occur when a large generic constant `N` is selected.
//! Even if we use `Box::new(Set<BigT, 4096>)`, there is still a risk of stack-overflow
//! unless Rust provides a usable Placement-New feature.
//!
//! Regarding memory usage, in addition to the inserted key-value pairs (or items),
//! each [`Map`] or [`Set`] only occupies an additional `usize` of memory to store
//! quantity (len) information, that's all.

#![cfg_attr(all(not(feature = "std"), not(doc), not(test)), no_std)]
#![doc(html_root_url = "https://docs.rs/micromap/0.0.0")]
#![deny(warnings)]
// `clippy::pedantic` have false positives about the `#[must_use]` attr
// sometimes If a false positive occurs, we explicitly mark it
// `#[allow(clippy::must_use_candidate)]` separately.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::must_use_candidate)]
#![warn(rust_2018_idioms)]
// About the docs
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![warn(rustdoc::missing_crate_level_docs)]
// Our Goal, uncomment these! (now we hit it.)
#![warn(missing_docs)]
#![doc(test(attr(deny(unused))))]

pub mod map;
pub mod set;

// re-export Set
pub use map::Map;
pub use set::Set;
