// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use super::difference::Difference;
use super::Set;

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or `other` but not in both.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let a = Set::from([1, 2, 3]);
    /// let b = Set::from([4, 2, 3, 4]);
    /// // Can be seen as `(a - b) ∪ (b - a)`.
    /// for x in a.symmetric_difference(&b) {
    ///     println!("{x}"); // Print 1, 4
    /// }
    /// let sym_diff: Set<_, 7> = a.symmetric_difference(&b).copied().collect();
    /// assert_eq!(sym_diff, Set::from([1, 4]));
    /// ```
    #[inline]
    pub fn symmetric_difference<'a, const M: usize>(
        &'a self,
        other: &'a Set<T, M>,
    ) -> SymmetricDifference<'a, T, N, M> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }
}

/// A lazy iterator producing elements in the symmetric difference of Linear `Set`s.
///
/// This `struct` is created by the [`symmetric_difference`] method on [`Set`].
///
/// [`symmetric_difference`]: Set::symmetric_difference
///
/// # Examples
/// ```
/// use micromap::Set;
/// let a = Set::from([1, 2, 3]);
/// let b = Set::from([4, 2, 3, 4]);
/// let sym_diff = a.symmetric_difference(&b);
/// assert_eq!(sym_diff.count(), 2);
/// ```
#[must_use = "this returns the difference as an iterator, without modifying \
              either input set"]
pub struct SymmetricDifference<'a, T, const N: usize, const M: usize> {
    iter: core::iter::Chain<Difference<'a, 'a, T, M>, Difference<'a, 'a, T, N>>,
}

impl<T, const N: usize, const M: usize> Clone for SymmetricDifference<'_, T, N, M> {
    #[inline]
    fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T: PartialEq, const N: usize, const M: usize> Iterator
    for SymmetricDifference<'a, T, N, M>
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<T: PartialEq + core::fmt::Debug, const N: usize, const M: usize> core::fmt::Debug
    for SymmetricDifference<'_, T, N, M>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T: PartialEq, const N: usize, const M: usize> core::iter::FusedIterator
    for SymmetricDifference<'_, T, N, M>
{
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn symmetric_difference_simple() {
        let set_a = Set::from([0, 1, 2, 3, 5, 7, 9]);
        let set_b = Set::from([2, 5, 6, 7, 8, 10]);
        let set_c = Set::from([0, 1, 2, 3, 5, 7, 9, 2, 5, 6, 7, 8]);
        let set_result: Set<_, 13> = set_a.symmetric_difference(&set_b).copied().collect();
        assert_eq!(set_result, Set::from([0, 1, 3, 6, 8, 9, 10]));
        let set_result: Set<_, 19> = set_a.symmetric_difference(&set_c).copied().collect();
        assert_eq!(set_result, Set::from([6, 8]));
        let set_result: Set<_, 18> = set_b.symmetric_difference(&set_c).copied().collect();
        assert_eq!(set_result, Set::from([0, 1, 3, 9, 10]));
    }

    #[test]
    fn symmetric_difference_with_empty_set() {
        let set_a = Set::from([1, 2, 3]);
        let set_b: Set<i32, 3> = Set::new();
        let sym_diff: Set<_, 6> = set_a.symmetric_difference(&set_b).copied().collect();
        assert_eq!(sym_diff, set_a);
    }

    #[test]
    fn symmetric_difference_with_disjoint_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([4, 5, 6]);
        let sym_diff: Set<_, 6> = a.symmetric_difference(&b).copied().collect();
        assert_eq!(sym_diff, Set::from([1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn symmetric_difference_with_subset() {
        let a = Set::from([1, 2, 3, 4]);
        let b = Set::from([2, 3]);
        let sym_diff: Set<_, 6> = a.symmetric_difference(&b).copied().collect();
        assert_eq!(sym_diff, Set::from([1, 4]));
    }

    #[test]
    fn symmetric_difference_with_superset() {
        let a = Set::from([2, 3]);
        let b = Set::from([1, 2, 3, 4]);
        let sym_diff: Set<_, 6> = a.symmetric_difference(&b).copied().collect();
        assert_eq!(sym_diff, Set::from([1, 4]));
    }

    #[test]
    fn symmetric_difference_size_hint() {
        let set_a = Set::from([1, 1, 2, 3]); // cap is 4, but len() is 3
        let set_b = Set::from([4, 5, 6, 6, 6, 7, 8, 9]); // cap is 8, but len() is 6
        let set_c = Set::from([]);
        let set_d = Set::from([3, 4]);
        assert_eq!(set_a.symmetric_difference(&set_b).size_hint(), (3, Some(9)));
        assert_eq!(set_a.symmetric_difference(&set_c).size_hint(), (3, Some(3)));
        assert_eq!(set_a.symmetric_difference(&set_d).size_hint(), (1, Some(5)));
        assert_eq!(set_b.symmetric_difference(&set_a).size_hint(), (3, Some(9)));
        assert_eq!(set_b.symmetric_difference(&set_d).size_hint(), (4, Some(8)));
        assert_eq!(set_c.symmetric_difference(&set_b).size_hint(), (6, Some(6)));
        assert_eq!(set_d.symmetric_difference(&set_a).size_hint(), (1, Some(5)));
        assert_eq!(set_d.symmetric_difference(&set_b).size_hint(), (4, Some(8)));
        assert_eq!(set_d.symmetric_difference(&set_c).size_hint(), (2, Some(2)));
    }

    #[test]
    fn symmetric_difference_clone() {
        let set_a = Set::from([1, 2, 3]);
        let set_b = Set::from([3, 4, 5]);
        let sym_diff = set_a.symmetric_difference(&set_b);
        let cloned = sym_diff.clone();
        let collected_original: Set<_, 6> = sym_diff.copied().collect();
        let collected_cloned: Set<_, 6> = cloned.copied().collect();
        assert_eq!(collected_original, collected_cloned);
    }

    #[test]
    fn symmetric_difference_fold() {
        let set_a = Set::from([1, 2, 3]);
        let set_b = Set::from([3, 4, 5]);
        let sym_diff = set_a.symmetric_difference(&set_b);
        let sum = sym_diff.fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 1 + 2 + 4 + 5);
    }

    #[test]
    fn symmetric_difference_fmt_debug() {
        let set_a = Set::from([1, 2, 3]);
        let set_b = Set::from([3, 4, 5]);
        let sym_diff = set_a.symmetric_difference(&set_b);
        let debug_output = format!("{:?}", sym_diff);
        assert!(debug_output.contains("1"));
        assert!(debug_output.contains("2"));
        assert!(debug_output.contains("4"));
        assert!(debug_output.contains("5"));
        assert!(!debug_output.contains("3"));
    }
}
