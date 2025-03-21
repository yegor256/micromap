// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<K: PartialEq, V: PartialEq, const N: usize> PartialEq for Map<K, V, N> {
    /// Two maps can be compared.
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Map<u8, i32, 10> = micromap::Map::new();
    /// let mut m2: micromap::Map<u8, i32, 10> = micromap::Map::new();
    /// m1.insert(1, 42);
    /// m2.insert(1, 42);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// // two maps with different order of key-value pairs are still equal:
    /// m1.insert(2, 1);
    /// m1.insert(3, 16);
    /// m2.insert(3, 16);
    /// m2.insert(2, 1);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|(k, v)| other.get(k) == Some(v))
    }
}

impl<K: Eq, V: Eq, const N: usize> Eq for Map<K, V, N> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compares_two_maps() {
        let mut m1: Map<String, i32, 10> = Map::new();
        m1.insert("first".to_string(), 42);
        let mut m2: Map<String, i32, 10> = Map::new();
        m2.insert("first".to_string(), 42);
        assert!(m1.eq(&m2));
    }
}
