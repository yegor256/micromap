// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<K: PartialEq, V: PartialEq, const N: usize, const M: usize> PartialEq<Map<K, V, M>>
    for Map<K, V, N>
{
    /// Two maps can be compared. (The capacity does not affect comparison.)
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Map<u8, i32, 5> = micromap::Map::new();
    /// let mut m2: micromap::Map<u8, i32, 10> = micromap::Map::new();
    /// m1.insert(1, 42);
    /// m2.insert(1, 42);
    ///
    /// assert_eq!(m1, m2);
    /// // two maps with different order of key-value pairs are still equal:
    /// m1.insert(2, 1);
    /// m1.insert(3, 16);
    /// m2.insert(3, 16);
    /// m2.insert(2, 1);
    ///
    /// assert_eq!(m1, m2);
    /// ```
    #[inline]
    fn eq(&self, other: &Map<K, V, M>) -> bool {
        self.len() == other.len() && self.iter().all(|(k, v)| other.get(k) == Some(v))
    }
}

impl<K: Eq, V: Eq, const N: usize> Eq for Map<K, V, N> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compares_two_maps() {
        let mut m1: Map<String, i32, 5> = Map::new();
        m1.insert("first".to_string(), 42);
        let mut m2: Map<String, i32, 5> = Map::new();
        m2.insert("first".to_string(), 42);
        assert!(m1.eq(&m2));
    }

    #[test]
    fn compares_two_diff_cap_maps() {
        let mut m1: Map<char, i32, 3> = Map::from([('a', 97), ('b', 98), ('c', 99)]);
        let mut m2: Map<char, i32, 4> = Map::from([('c', 99), ('c', 99), ('c', 99), ('b', 98)]);
        m2.insert('a', 97);
        assert!(m1.eq(&m2));
        m1.remove(&'c');
        assert!(m1.ne(&m2));
    }
}
