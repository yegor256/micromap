// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Set;

impl<T: PartialEq, const N: usize> PartialEq for Set<T, N> {
    /// Two sets can be compared.
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Set<u8, 10> = micromap::Set::new();
    /// let mut m2: micromap::Set<u8, 10> = micromap::Set::new();
    /// m1.insert(1);
    /// m2.insert(1);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// // two sets with different order of key-value pairs are still equal:
    /// m1.insert(2);
    /// m1.insert(3);
    /// m2.insert(3);
    /// m2.insert(2);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}

impl<T: Eq, const N: usize> Eq for Set<T, N> {}
