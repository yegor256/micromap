// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use super::Set;
use core::ops::Sub;

impl<T, const N: usize, const M: usize> Sub<&Set<T, M>> for &Set<T, N>
where
    T: PartialEq + Clone,
{
    type Output = Set<T, N>;

    /// Returns the difference of `self` and `rhs` as a new `Set<T, N>`.
    /// The capacity of return set is same as `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let a = Set::from([0, 1, 2, 3, 4]);
    /// let b = Set::from([1, 3, 4, 5]);
    /// let set = &a - &b;
    /// let expected = Set::from([0, 2]);
    ///
    /// assert_eq!(set, expected);
    /// ```
    fn sub(self, rhs: &Set<T, M>) -> Set<T, N> {
        self.difference(rhs).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Set;

    #[test]
    fn test_sub_with_non_overlapping_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([4, 5, 6]);
        let result = &a - &b;
        let expected = [1, 2, 3];
        let mut i = 0;
        for x in &result {
            assert!(expected.contains(x));
            i += 1;
        }
        assert_eq!(i, expected.len());
    }

    #[test]
    fn test_sub_with_overlapping_sets() {
        let a = Set::from([1, 2, 3, 4]);
        let b = Set::from([3, 4, 5]);
        let result = &a - &b;
        let expected = [1, 2];
        assert_eq!(
            expected.len(),
            result.iter().fold(0, |acc, x| {
                assert!(expected.contains(x));
                acc + 1
            })
        );
    }

    #[test]
    fn test_sub_with_empty_rhs() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([]);
        let result = &a - &b;
        let expected = Set::from([1, 2, 3, 3, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_with_empty_lhs() {
        let a = Set::from([]);
        let b = Set::from([1, 2, 3]);
        let result = &a - &b;
        assert!(result.is_empty());
    }

    #[test]
    fn test_sub_with_identical_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([1, 2, 3]);
        let result = &a - &b;
        assert!(result.is_empty());
    }
}
