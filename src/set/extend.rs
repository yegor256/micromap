// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use super::Set;

impl<T: PartialEq, const N: usize> Extend<T> for Set<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|item| {
            self.insert(item);
        });
    }
}

impl<'a, T: 'a + PartialEq + Copy, const N: usize> Extend<&'a T> for Set<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn extend_set_empty() {
        let mut set = Set::<i32, 6>::new();
        set.extend([1, 2, 3]);
        assert_eq!(set, Set::from([1, 2, 3]));
    }

    #[test]
    fn extend_set_not_empty() {
        let mut set = Set::<i32, 6>::from_iter([1, 2]);
        set.extend([1, 2, 3]);
        assert_eq!(set, Set::from([1, 2, 3]));
    }

    #[test]
    fn extend_set_overlap() {
        let mut set = Set::<i32, 6>::from_iter([1, 2, 4]);
        set.extend([2, 3, 5, 6]);
        assert_eq!(set, Set::from([1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn extend_set_with_str_references() {
        let mut set = Set::<&str, 6>::new();
        set.extend(["a", "b", "c"].iter());
        assert_eq!(set, Set::from(["a", "b", "c"]));
    }

    #[test]
    fn extend_set_with_str_references_overlap() {
        let mut set = Set::<&str, 6>::from_iter(["a", "b"]);
        set.extend(["b", "c", "d"].iter());
        assert_eq!(set, Set::from(["a", "b", "c", "d"]));
    }

    #[test]
    fn extend_set_with_str_references_empty() {
        let mut set = Set::<&str, 6>::from_iter(["a", "b", "c"]);
        <Set<&str, 6> as Extend<&str>>::extend(&mut set, []); // Fully Qualified Syntax
        assert_eq!(set, Set::from(["a", "b", "c"]));
    }
}
