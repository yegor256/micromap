// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{IntoIter, Iter, IterMut, Map};
use core::iter::FusedIterator;

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<K, V> {
        self.into_iter()
    }

    /// An iterator with mutable references to the values but
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.into_iter()
    }
}

impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| {
            let p = unsafe { p.assume_init_ref() };
            (&p.0, &p.1)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| {
            let p = unsafe { p.assume_init_mut() };
            (&p.0, &mut p.1)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<K: PartialEq, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.map.len > 0 {
            self.map.len -= 1;
            Some(self.map.item_read(self.map.len))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.map.len, Some(self.map.len))
    }

    #[inline]
    fn count(self) -> usize {
        self.map.len()
    }
}

impl<'a, K: PartialEq, V, const N: usize> IntoIterator for &'a Map<K, V, N> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.pairs[0..self.len].iter(),
        }
    }
}

impl<'a, K: PartialEq, V, const N: usize> IntoIterator for &'a mut Map<K, V, N> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.pairs[0..self.len].iter_mut(),
        }
    }
}

impl<K: PartialEq, V, const N: usize> IntoIterator for Map<K, V, N> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { map: self }
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: PartialEq, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {
    fn len(&self) -> usize {
        self.map.len
    }
}

impl<K, V> FusedIterator for Iter<'_, K, V> {}

impl<K, V> FusedIterator for IterMut<'_, K, V> {}

impl<K: PartialEq, V, const N: usize> FusedIterator for IntoIter<K, V, N> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_iterator() {
        let m: Map<u32, u32, 4> = Map::new();
        assert!(m.into_iter().next().is_none());
    }

    #[test]
    fn insert_and_jump_over_next() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 42);
        let mut iter = m.into_iter();
        assert_eq!(42, iter.next().unwrap().1);
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert_and_iterate() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let mut sum = 0;
        for (_k, v) in m.iter() {
            sum += v;
        }
        assert_eq!(58, sum);
    }

    #[test]
    fn insert_and_into_iterate() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let mut sum = 0;
        for p in &m {
            sum += p.1;
        }
        assert_eq!(58, sum);
    }

    #[test]
    fn iterate_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        m.remove("two");
        let mut sum = 0;
        for (_k, v) in m.iter() {
            sum += v;
        }
        assert_eq!(6, sum);
    }

    #[test]
    fn into_iterate_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        m.remove("two");
        let mut sum = 0;
        for (_k, v) in m {
            sum += v;
        }
        assert_eq!(6, sum);
    }

    #[test]
    fn change_with_iter_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 2);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        for (_k, v) in m.iter_mut() {
            *v *= 2;
        }
        let sum = m.iter().map(|p| p.1).sum::<i32>();
        assert_eq!(20, sum);
    }

    #[test]
    fn iter_mut_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        assert_eq!(m.iter_mut().count(), 3);
        m.remove("two");
        assert_eq!(m.iter_mut().count(), 2);
        assert_eq!(m.iter_mut().last().unwrap().1, &5);
    }

    #[test]
    fn into_iter_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 2);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        for (_k, v) in &mut m {
            *v *= 2;
        }
        let sum = m.iter().map(|p| p.1).sum::<i32>();
        assert_eq!(20, sum);
    }

    #[test]
    fn into_iter_drop() {
        use std::rc::Rc;
        let mut m: Map<i32, Rc<()>, 8> = Map::new();
        let v = Rc::new(());
        let n = 8;
        for i in 0..n {
            m.insert(i, Rc::clone(&v));
        }
        assert_eq!(Rc::strong_count(&v), (n + 1) as usize);
        let _p = m.into_iter().nth(3);
        assert_eq!(Rc::strong_count(&v), 2); // v & p
    }

    #[test]
    fn iter_size_hint() {
        let mut m: Map<char, u32, 4> = Map::new();
        m.insert('a', 97);
        m.insert('c', 99);
        let it = m.iter();
        assert_eq!(it.len(), 2);
        let mut it_mut = m.iter_mut();
        assert!(it_mut.next().is_some());
        assert_eq!(it_mut.len(), 1);
        assert_eq!(it_mut.len(), it_mut.size_hint().0);
        let mut it_into = m.into_iter();
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_none());
        assert!(it_into.next().is_none());
        assert_eq!(it_into.len(), 0);
    }
}
