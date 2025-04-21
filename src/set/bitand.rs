// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use super::Set;

// If we can use `#![feature(generic_const_exprs)]`, use `Set<T, min(N, M)>` as the
// const generic parameter to replace the `Set<T, N>`.
impl<T, const M: usize, const N: usize> core::ops::BitAnd<&Set<T, M>> for &Set<T, N>
where
    T: PartialEq + Clone,
{
    type Output = Set<T, N>;

    /// Returns the intersection of `self` and `rhs` as a new `Set<T, N>`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let a = Set::from([1, 2, 3]);
    /// let b = Set::from([2, 3, 4, 5]);
    /// let set = &a & &b;
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &Set<T, M>) -> Set<T, N> {
        self.intersection(rhs).cloned().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn bitand_with_non_empty_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([2, 3, 4, 5]);
        let set = &a & &b;
        let mut i = 0;
        let expected = [2, 3];
        for x in &set {
            assert!(expected.contains(x));
            i += 1;
        }
        assert_eq!(i, expected.len());
    }

    #[test]
    fn bitand_with_disjoint_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([4, 5, 6]);
        let set = &a & &b;
        assert!(set.is_empty());
    }

    #[test]
    fn bitand_with_empty_set() {
        let a = Set::from([1, 2, 3]);
        let b: Set<i32, 0> = Set::new();
        let set = &a & &b;
        assert!(set.is_empty());
    }

    #[test]
    fn bitand_with_self() {
        let a = Set::from([1, 2, 3]);
        let set = &a & &a;
        let mut i = 0;
        let expected = [1, 2, 3];
        for x in &set {
            assert!(expected.contains(x));
            i += 1;
        }
        assert_eq!(i, expected.len());
    }

    #[test]
    fn bitand_with_subset() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([2, 3]);
        let set = &a & &b;
        let mut i = 0;
        let expected = [2, 3];
        for x in &set {
            assert!(expected.contains(x));
            i += 1;
        }
        assert_eq!(i, expected.len());
    }
}
