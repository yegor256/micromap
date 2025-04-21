// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;
use crate::map::Map;

impl<T, const N: usize> Default for Set<T, N> {
    /// Make a default empty [`Set`].
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Set<T, N> {
    /// Creates an empty `Set`.
    ///
    /// The set is initially created with a capacity of N, so even if you
    /// don't insert any values, it will occupy a fixed stack memory space.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let set: Set<i32, 8> = Set::new();
    /// assert_eq!(set.len(), 0);
    /// assert_eq!(set.capacity(), 8);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: Map::<T, (), N>::new(),
        }
    }
}
