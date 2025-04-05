// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, Set};

impl<T, const N: usize> Default for Set<T, N> {
    /// Make a default empty [`Set`].
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Set<T, N> {
    /// Make it.
    ///
    /// The size of the set is defined by the generic argument. For example,
    /// this is how you make a set of four key-values pairs:
    #[inline]
    #[must_use]
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new() -> Self {
        Self {
            map: Map::<T, (), N>::new(),
        }
    }
}
