// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use crate::Set;
use crate::SetIter;

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let a = Set::from([1, 2, 3]);
    /// let b = Set::from([4, 2, 3, 4]);
    ///
    /// // Can be seen as `a - b`.
    /// for x in a.difference(&b) {
    ///     println!("{x}"); // Print 1
    /// }
    ///
    /// let diff: Set<_, 3> = a.difference(&b).copied().collect();
    /// assert_eq!(diff, Set::from([1]));
    ///
    /// // Note that difference is not symmetric,
    /// // and `b - a` means something else:
    /// let diff: Set<_, 4> = b.difference(&a).copied().collect();
    /// assert_eq!(diff, Set::from([4]));
    /// ```
    #[inline]
    pub fn difference<'a, const M: usize>(&'a self, other: &'a Set<T, M>) -> Difference<'a, T, M> {
        Difference {
            iter: self.iter(),
            other,
        }
    }
}

/// A lazy iterator producing elements in the difference of Linear `Set`s.
///
/// This `struct` is created by the [`difference`] method on [`Set`].
///
/// [`difference`]: Set::difference
///
/// # Examples
///
/// ```
/// use micromap::Set;
///
/// let a = Set::from([1, 2, 3]);
/// let b = Set::from([4, 2, 3, 4]);
///
/// let mut difference = a.difference(&b);
/// ```
pub struct Difference<'a, T: 'a + PartialEq, const M: usize> {
    // iterator of the first set
    iter: SetIter<'a, T>,
    // the second set
    other: &'a Set<T, M>,
}

impl<T: PartialEq, const M: usize> Clone for Difference<'_, T, M> {
    #[inline]
    fn clone(&self) -> Self {
        Difference {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a, T: PartialEq, const M: usize> Iterator for Difference<'a, T, M> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&item| !self.other.contains(item))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        // Maybe using iterator is better than the default Iterator::fold() which uses while loop.
        self.iter.fold(init, |acc, elt| {
            if self.other.contains(elt) {
                acc
            } else {
                f(acc, elt)
            }
        })
    }
}

impl<T: core::fmt::Debug + PartialEq, const M: usize> core::fmt::Debug for Difference<'_, T, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[cfg(test)]
mod tests {

    use crate::Set;

    // NOTE: This is a BUG in the standard library function.
    // #[test]
    // #[ignore]
    // fn difference_lifetime() {
    //     use std::collections::hash_set::HashSet as Set;
    //
    //     let sentence_1 = String::from("I love the surf and the sand.");
    //     let sentence_1_words: Set<&str> = sentence_1.split(" ").collect();
    //
    //     let first_only = {
    //         let sentence_2 = String::from("I hate the hate and the sand.");
    //         let sentence_2_words: Set<&str> = sentence_2.split(" ").collect();
    //         let first_only: Vec<_> = sentence_1_words.difference(&sentence_2_words).collect();
    //         let second_only: Vec<_> = sentence_2_words.difference(&sentence_1_words).collect();
    //
    //         println!("First  Sentence: {}", sentence_1);
    //         println!("Second Sentence: {}", sentence_2);
    //         println!("{:?}", first_only);
    //         println!("{:?}", second_only);
    //         first_only
    //     };
    //
    //     assert_eq!(
    //         Set::<_>::from(first_only.into_iter().copied()),
    //         Set::<_>::from(["love", "surf"])
    //     );
    // }

    #[test]
    fn difference_disjoint() {
        let set_a: Set<u32, 5> = Set::from([0, 1, 3, 5, 7]);
        let set_b: Set<u32, 4> = Set::from([2, 4, 6, 8]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<u32, 5>>();
        assert_eq!(set_a, set_diff);
    }

    #[test]
    fn difference_with_overlap() {
        let set_a = Set::from([1, 3, 5, 7]);
        let set_b = Set::from([3, 5, 6, 8, 9]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<u32, 4>>();
        let expected = Set::from([1, 7]);
        assert_eq!(expected, set_diff);
    }

    #[test]
    fn difference_complete_overlap() {
        let set_a = Set::from([1, 3, 5, 7]);
        let set_b = Set::from([1, 3, 5, 7]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<u32, 4>>();
        let expected = Set::from([]);
        assert_eq!(expected, set_diff);
    }

    #[test]
    fn difference_empty_set() {
        let set_a = Set::from([1, 3, 5, 7]);
        let set_b = Set::from([]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<u32, 4>>();
        assert_eq!(set_a, set_diff);
    }

    #[test]
    fn difference_with_empty_first_set() {
        let set_a = Set::from([]);
        let set_b = Set::from([2, 4, 6, 8]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<u32, 4>>();
        let expected = Set::from([]);
        assert_eq!(expected, set_diff);
    }

    #[test]
    fn difference_partial_overlap() {
        let set_a = Set::from([1, 2, 3, 4, 5, 6]);
        let set_b = Set::from([4, 5, 6, 7, 8, 9]);

        let set_diff = set_a.difference(&set_b).copied().collect::<Set<_, 6>>();
        let expected = Set::from([1, 2, 3]);
        assert_eq!(expected, set_diff);
    }
}
