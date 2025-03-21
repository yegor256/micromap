// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<K: PartialEq, V, const N: usize> FromIterator<(K, V)> for Map<K, V, N> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut m: Self = Self::new();
        for (k, v) in iter {
            m.insert(k, v);
        }
        m
    }
}

impl<K: PartialEq, V, const N: usize> From<[(K, V); N]> for Map<K, V, N> {
    #[inline]
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_ARRAY: [(i32, &str); 5] =
        [(1, "sun"), (2, "mon"), (3, "tue"), (4, "wed"), (5, "thu")];

    #[test]
    fn from_iter() {
        let vec = Vec::from(TEST_ARRAY);
        let m: Map<i32, &str, 10> = Map::from_iter(vec);
        assert_eq!(m.len(), 5);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn from_larger_iter() {
        let vec = Vec::from(TEST_ARRAY);
        let _m: Map<i32, &str, 1> = Map::from_iter(vec);
    }

    #[test]
    fn from_array() {
        let m = Map::from(TEST_ARRAY);
        assert_eq!(m.len(), 5);
    }

    #[test]
    fn array_into_map() {
        let m: Map<i32, &str, 5> = TEST_ARRAY.into();
        assert_eq!(m.len(), 5);
    }

    #[test]
    fn from_with_duplicates() {
        let arr = [(1, "sun"), (2, "mon"), (3, "tue"), (1, "wed"), (2, "thu")];
        let m = Map::from(arr);
        assert_eq!(m.len(), 3);
        assert_eq!(m[&2], "thu");
    }
}
