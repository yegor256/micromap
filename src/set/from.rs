// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;

impl<T: PartialEq, const N: usize> FromIterator<T> for Set<T, N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s: Self = Self::new();
        for k in iter {
            s.insert(k);
        }
        s
    }
}

impl<T: PartialEq, const N: usize> From<[T; N]> for Set<T, N> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}
#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn test_from_iter() {
        let vec = vec![1, 2, 3, 4];
        let set: Set<_, 4> = vec.into_iter().collect();
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(!set.contains(&5));
    }

    #[test]
    fn test_from_array() {
        let arr = [1, 2, 3, 4];
        let set: Set<_, 4> = Set::from(arr);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(!set.contains(&5));
    }
}
