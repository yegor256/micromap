// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use core::borrow::Borrow;
use core::ops::{Index, IndexMut};

impl<K: PartialEq + Borrow<Q>, Q: PartialEq + ?Sized, V, const N: usize> Index<&Q>
    for Map<K, V, N>
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("No entry found for the key")
    }
}

impl<K: PartialEq + Borrow<Q>, Q: PartialEq + ?Sized, V, const N: usize> IndexMut<&Q>
    for Map<K, V, N>
{
    #[inline]
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("No entry found for the key")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn index() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("first".to_string(), 42);
        assert_eq!(m["first"], 42);
    }

    #[test]
    fn index_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("first".to_string(), 42);
        m["first"] += 10;
        assert_eq!(m["first"], 52);
    }

    #[test]
    #[should_panic]
    fn wrong_index() -> () {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("first".to_string(), 42);
        assert_eq!(m["second"], 42);
    }

    #[cfg(test)]
    #[derive(PartialEq)]
    struct Container {
        pub t: i32,
    }

    #[cfg(test)]
    impl Borrow<i32> for Container {
        fn borrow(&self) -> &i32 {
            &self.t
        }
    }

    #[test]
    fn index_by_borrow() {
        let mut m: Map<Container, i32, 10> = Map::new();
        m.insert(Container { t: 10 }, 42);
        assert_eq!(m[&10], 42);
    }
}
