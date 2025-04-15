// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! This is a simpler and faster alternative implementation of the standard `HashMap`.
//! It doesn't use heap and doesn't use hashing at all. It simply keeps all key-value
//! pairs in an array and when it's necessary to retrieve by key, it scrolls through
//! the entire array. This implementation works much faster for small maps of
//! less than 50 keys, but definitely is not suitable for larger maps.
//!
//! Check [this page](https://github.com/yegor256/micromap#benchmark)
//! for the recent benchmarking results.
//!
//! For example, here is how a map with a few keys can be created:
//!
//! ```
//! use micromap::Map;
//! let mut m : Map<u64, &str, 10> = Map::new();
//! m.insert(1, "Hello, world!");
//! m.insert(2, "Good bye!");
//! # #[cfg(std)]
//! assert_eq!(2, m.len());
//! ```
//!
//! Creating a [`Map`] requires knowing the maximum size of it, upfront. This is
//! what the third type argument `10` is for, in the example above. The array
//! will have exactly ten elements. An attempt to add an 11th element will lead
//! to a panic.

#![cfg_attr(all(not(feature = "std"), not(doc), not(test)), no_std)]
#![doc(html_root_url = "https://docs.rs/micromap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(rust_2018_idioms)]
// About the docs
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![warn(rustdoc::missing_crate_level_docs)]
// Our Goal, uncomment these!
// #![warn(missing_docs)]
// #![doc(test(attr(deny(unused))))]
#![doc(test(attr(warn(unused))))]

mod clone;
mod ctors;
mod debug;
mod display;
mod drain;
mod entry;
mod eq;
mod from;
mod index;
mod iterators;
mod keys;
mod map;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serialization;
mod set;
mod values;

pub use drain::Drain;
pub use iterators::{IntoIter, Iter, IterMut};
pub use set::{Set, SetDrain, SetIntoIter, SetIter};

use core::mem::MaybeUninit;

/// A faster alternative of [`std::collections::HashMap`].
///
/// For example, this is how you make a map, which is allocated on stack and is capable of storing
/// up to eight key-values pairs:
///
/// ```
/// let mut m : micromap::Map<u64, &str, 8> = micromap::Map::new();
/// m.insert(1, "Jeff Lebowski");
/// m.insert(2, "Walter Sobchak");
/// # #[cfg(std)]
/// assert_eq!(2, m.len());
/// ```
///
/// It is faster because it doesn't use a hash function at all. It simply keeps
/// all pairs in an array and when it's necessary to find a value, it goes through
/// all pairs comparing the needle with each pair available. Also it is faster
/// because it doesn't use heap. When a [`Map`] is being created, it allocates the necessary
/// space on stack. That's why the maximum size of the map must be provided in
/// compile time.
///
/// It is also faster because it doesn't grow in size. When a [`Map`] is created,
/// its size is fixed on stack. If an attempt is made to insert too many keys
/// into it, it simply panics. Moreover, in the "release" mode it doesn't panic,
/// but its behaviour is undefined. In the "release" mode all boundary checks
/// are disabled, for the sake of higher performance.
pub struct Map<K, V, const N: usize> {
    /// The next available pair in the array.
    len: usize,
    /// The fixed-size array of key-value pairs.
    pairs: [MaybeUninit<(K, V)>; N],
}

/// An iterator over the values of the [`Map`].
#[repr(transparent)]
pub struct Values<'a, K, V> {
    iter: Iter<'a, K, V>,
}

/// Mutable iterator over the values of the [`Map`].
#[repr(transparent)]
pub struct ValuesMut<'a, K, V> {
    iter: IterMut<'a, K, V>,
}

/// Consuming iterator over the values of the [`Map`].
#[repr(transparent)]
pub struct IntoValues<K, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}

/// A read-only iterator over the keys of the [`Map`].
#[repr(transparent)]
pub struct Keys<'a, K, V> {
    iter: Iter<'a, K, V>,
}

/// Consuming iterator over the keys of the [`Map`].
#[repr(transparent)]
pub struct IntoKeys<K, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`Map`].
///
/// [`entry`]: Map::entry
pub enum Entry<'a, K, V, const N: usize> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, N>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, N>),
}

/// A view into an occupied entry in a `Map`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K, V, const N: usize> {
    index: usize,
    table: &'a mut Map<K, V, N>,
}

/// A view into a vacant entry in a `Map`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K, V, const N: usize> {
    key: K,
    table: &'a mut Map<K, V, N>,
}
