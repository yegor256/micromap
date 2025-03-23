// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use crate::Set;

impl<T: PartialEq, const N: usize, const M: usize> PartialEq<Set<T, M>> for Set<T, N> {
    /// Two sets can be compared. (The capacity does not affect comparison.)
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Set<_, 5> = micromap::Set::new();
    /// let mut m2: micromap::Set<_, 10> = micromap::Set::new();
    /// m1.insert(1);
    /// m2.insert(1);
    ///
    /// assert_eq!(m1, m2);
    /// // two sets with different order of key-value pairs are still equal:
    /// m1.insert(2);
    /// m1.insert(3);
    /// m2.insert(3);
    /// m2.insert(2);
    ///
    /// assert_eq!(m1, m2);
    /// ```
    #[inline]
    fn eq(&self, other: &Set<T, M>) -> bool {
        self.map.eq(&other.map)
    }
}

impl<T: Eq, const N: usize> Eq for Set<T, N> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compares_two_sets() {
        let mut s1: Set<i32, 1> = Set::new();
        s1.insert(1);
        let mut s2: Set<i32, 1> = Set::new();
        s2.insert(1);
        assert_eq!(s1, s2);
    }

    #[test]
    fn compares_sets_with_different_order() {
        let mut s1: Set<i32, 3> = Set::new();
        s1.insert(1);
        s1.insert(2);
        s1.insert(3);

        let mut s2: Set<i32, 3> = Set::new();
        s2.insert(3);
        s2.insert(2);
        s2.insert(1);

        assert_eq!(s1, s2);
    }

    #[test]
    fn compares_sets_with_different_lengths() {
        let mut s1: Set<i32, 3> = Set::new();
        s1.insert(1);
        s1.insert(2);

        let mut s2: Set<i32, 3> = Set::new();
        s2.insert(1);

        assert_ne!(s1, s2);
    }

    #[test]
    fn compares_sets_with_different_elements() {
        let mut s1: Set<i32, 1> = Set::new();
        s1.insert(1);

        let mut s2: Set<i32, 1> = Set::new();
        s2.insert(2);

        assert_ne!(s1, s2);
    }

    #[test]
    fn compares_sets_with_char_elements() {
        let mut s1: Set<char, 2> = Set::new();
        s1.insert('a');
        s1.insert('b');

        let mut s2: Set<char, 3> = Set::new();
        s2.insert('a');
        s2.insert('b');

        assert_eq!(s1, s2);
    }
}
