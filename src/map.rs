// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! A small Map based on a fixed length array which stores key-value pairs directly.

mod clone;
mod ctors;
mod debug;
mod display;
pub(crate) mod drain;
mod entry;
mod eq;
mod from;
mod index;
mod iterators;
pub(crate) mod keys;
mod methods;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serialization;
mod values;

// re-export
pub use drain::Drain;
pub use entry::{Entry, OccupiedEntry, VacantEntry};
pub use iterators::{IntoIter, Iter, IterMut};
pub use keys::{IntoKeys, Keys};
pub use values::{IntoValues, Values, ValuesMut};

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
/// assert_eq!(m.len(), 2);
/// assert_eq!(m.capacity(), 8);
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
