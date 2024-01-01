// Copyright (c) 2023-2024 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{IntoIter, Iter, IterMut, Map};
use core::iter::FusedIterator;
use core::mem::ManuallyDrop;

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

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| {
            let p = unsafe { p.assume_init_ref() };
            (&p.0, &p.1)
        })
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
}

impl<K: PartialEq, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.map.len {
            let p = self.map.item_read(self.pos);
            self.pos += 1;
            Some(p)
        } else {
            None
        }
    }
}

impl<'a, K: PartialEq, V, const N: usize> IntoIterator for &'a Map<K, V, N> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    #[must_use]
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
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            pos: 0,
            map: ManuallyDrop::new(self),
        }
    }
}

impl<K: PartialEq, V, const N: usize> Drop for IntoIter<K, V, N> {
    fn drop(&mut self) {
        for i in self.pos..self.map.len {
            self.map.item_drop(i);
        }
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|p| {
            let p = unsafe { p.assume_init_ref() };
            (&p.0, &p.1)
        })
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|p| {
            let p = unsafe { p.assume_init_mut() };
            (&p.0, &mut p.1)
        })
    }
}

impl<K: PartialEq, V, const N: usize> DoubleEndedIterator for IntoIter<K, V, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos < self.map.len {
            self.map.len -= 1;
            let i = self.map.len;
            let p = self.map.item_read(i);
            Some(p)
        } else {
            None
        }
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: PartialEq, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {
    fn len(&self) -> usize {
        self.map.len - self.pos
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> {}

impl<K: PartialEq, V, const N: usize> FusedIterator for IntoIter<K, V, N> {}

#[cfg(test)]
mod test {

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
}
