// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use crate::set::difference::Difference;
use crate::Set;
use crate::SetIter;

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Visits the values representing the union,
    /// i.e., the values that are in `self` or `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let a = Set::from([1, 2, 3]);
    /// let b = Set::from([4, 2, 3, 4]);
    ///
    /// // Can be seen as `a ∪ b`.
    /// for x in a.union(&b) {
    ///     println!("{x}"); // Print 1, 2, 3, 4
    /// }
    ///
    /// let union: Set<_, 7> = a.union(&b).copied().collect();
    /// assert_eq!(union, Set::from([1, 2, 3, 4]));
    /// ```
    #[inline]
    pub fn union<'a, const M: usize>(&'a self, other: &'a Set<T, M>) -> Union<'a, T, M> {
        Union {
            iter: other.iter().chain(self.difference(other)),
        }
    }
}

/// A lazy iterator producing elements in the union of Linear `Set`s.
///
/// This `struct` is created by the [`union`] method on [`Set`].
///
/// [`union`]: Set::union
///
/// # Examples
///
/// ```
/// use micromap::Set;
///
/// let a = Set::from([1, 2, 3]);
/// let b = Set::from([4, 2, 3, 4]);
///
/// let mut union = a.union(&b);
/// ```
#[must_use = "this returns the union as an iterator, without modifying either input set"]
pub struct Union<'a, T: 'a + PartialEq, const M: usize> {
    iter: core::iter::Chain<SetIter<'a, T>, Difference<'a, T, M>>,
}

impl<T: PartialEq, const M: usize> Clone for Union<'_, T, M> {
    #[inline]
    fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T: PartialEq, const M: usize> Iterator for Union<'a, T, M> {
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
    fn count(self) -> usize {
        self.iter.count()
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

impl<T: core::fmt::Debug + PartialEq, const M: usize> core::fmt::Debug for Union<'_, T, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T: PartialEq, const M: usize> core::iter::FusedIterator for Union<'_, T, M> {}

#[cfg(test)]
mod tests {

    use crate::Set;

    #[test]
    fn union_simple() {
        let set_a = Set::from([0, 1, 2, 3, 5, 7, 9]);
        let set_b = Set::from([2, 5, 6, 7, 8, 10]);
        let set_c = Set::from([0, 1, 2, 3, 5, 7, 9, 2, 5, 6, 7, 8]);

        let set_result: Set<_, 13> = set_a.union(&set_b).copied().collect();
        assert_eq!(set_result, Set::from([0, 1, 2, 3, 5, 6, 7, 8, 9, 10]));
        let set_result: Set<_, 19> = set_a.union(&set_c).copied().collect();
        assert_eq!(set_result, Set::from([0, 1, 2, 3, 5, 6, 7, 8, 9]));
        let set_result: Set<_, 18> = set_b.union(&set_c).copied().collect();
        assert_eq!(set_result, Set::from([0, 1, 2, 3, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn union_with_empty_set() {
        let set_a = Set::from([1, 2, 3]);
        let set_b: Set<i32, 3> = Set::new();
        let union: Set<_, 6> = set_a.union(&set_b).copied().collect();
        assert_eq!(union, set_a);
    }

    #[test]
    fn union_with_disjoint_sets() {
        let a = Set::from([1, 2, 3]);
        let b = Set::from([4, 5, 6]);
        let union: Set<_, 6> = a.union(&b).copied().collect();
        assert_eq!(union, Set::from([1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn union_with_subset() {
        let a = Set::from([1, 2, 3, 4]);
        let b = Set::from([2, 3]);
        let union: Set<_, 6> = a.union(&b).copied().collect();
        assert_eq!(union, Set::from([1, 2, 3, 4]));
    }

    #[test]
    fn union_with_superset() {
        let a = Set::from([2, 3]);
        let b = Set::from([1, 2, 3, 4]);
        let union: Set<_, 6> = a.union(&b).copied().collect();
        assert_eq!(union, Set::from([1, 2, 3, 4]));
    }

    #[test]
    fn union_size_hint() {
        let set_a = Set::from([1, 1, 2, 3]); // cap is 4, but len() is 3
        let set_b = Set::from([4, 5, 6, 6, 6, 7, 8, 9]); // cap is 8, but len() is 6
        let set_c = Set::from([]);
        let set_d = Set::from([3, 4]);

        assert_eq!(set_a.union(&set_b).size_hint(), (6, Some(9)));
        assert_eq!(set_a.union(&set_c).size_hint(), (3, Some(3)));
        assert_eq!(set_a.union(&set_d).size_hint(), (3, Some(5)));

        assert_eq!(set_b.union(&set_a).size_hint(), (6, Some(9)));
        assert_eq!(set_b.union(&set_d).size_hint(), (6, Some(8)));

        assert_eq!(set_c.union(&set_b).size_hint(), (6, Some(6)));

        assert_eq!(set_d.union(&set_a).size_hint(), (3, Some(5)));
        assert_eq!(set_d.union(&set_b).size_hint(), (6, Some(8)));
        assert_eq!(set_d.union(&set_c).size_hint(), (2, Some(2)));
    }
}
