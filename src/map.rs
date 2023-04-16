// Copyright (c) 2023 Yegor Bugayenko
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

use crate::Pair::{Absent, Present};
use crate::{Map, MapIntoIter, Pair, MapIter};

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for MapIter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            if let Present(p) = &self.pairs[self.pos] {
                self.pos += 1;
                return Some((&p.0, &p.1));
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for MapIntoIter<'a, K, V, N> {
    type Item = (K, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            if self.pairs[self.pos].is_some() {
                let pair = self.pairs[self.pos].clone().unwrap();
                self.pos += 1;
                return Some(pair);
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Copy + PartialEq, V: Clone, const N: usize> IntoIterator for &'a Map<K, V, N> {
    type Item = (K, V);
    type IntoIter = MapIntoIter<'a, K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        MapIntoIter {
            pos: 0,
            pairs: &self.pairs,
        }
    }
}

impl<K: Copy + PartialEq, V: Clone, const N: usize> Default for Map<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Copy + PartialEq, V: Clone, const N: usize> Map<K, V, N> {
    /// Make it.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            pairs: [(); N].map(|_| Pair::<K, V>::default()),
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> MapIter<K, V, N> {
        MapIter {
            pos: 0,
            pairs: &self.pairs,
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> MapIntoIter<K, V, N> {
        MapIntoIter {
            pos: 0,
            pairs: &self.pairs,
        }
    }

    /// Is it empty?
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the total number of pairs inside.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let mut busy = 0;
        for i in 0..N {
            if self.pairs[i].is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Contains this key?
    #[inline]
    pub fn contains_key(&self, k: K) -> bool {
        for i in 0..N {
            if let Present((bk, _bv)) = &self.pairs[i] {
                if *bk == k {
                    return true;
                }
            }
        }
        false
    }

    /// Remove by key.
    #[inline]
    pub fn remove(&mut self, k: K) {
        for i in 0..N {
            if let Present((bk, _bv)) = &self.pairs[i] {
                if *bk == k {
                    self.pairs[i] = Absent;
                    break;
                }
            }
        }
    }

    /// Insert a single pair into it.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to insert too many pairs.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        self.remove(k);
        for i in 0..N {
            if !self.pairs[i].is_some() {
                self.pairs[i] = Present((k, v));
                return;
            }
        }
        panic!("There are only {N} pairs available in the map and all of them are already occupied")
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    pub fn get(&self, k: K) -> Option<&V> {
        for i in 0..N {
            if let Present(p) = &self.pairs[i] {
                if p.0 == k {
                    return Some(&p.1);
                }
            }
        }
        None
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Panics
    ///
    /// If can't turn it into a mutable state.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        for i in 0..N {
            if let Present(p) = &mut self.pairs[i] {
                if p.0 == k {
                    return Some(&mut self.pairs[i].as_mut().unwrap().1);
                }
            }
        }
        None
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn insert_and_check_length() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("first", 42);
    assert_eq!(1, m.len());
    m.insert("second", 16);
    assert_eq!(2, m.len());
    m.insert("first", 16);
    assert_eq!(2, m.len());
    Ok(())
}

#[test]
fn empty_length() -> Result<()> {
    let m: Map<u32, u32, 10> = Map::new();
    assert_eq!(0, m.len());
    Ok(())
}

#[test]
fn empty_iterator() -> Result<()> {
    let m: Map<u32, u32, 4> = Map::new();
    assert!(m.into_iter().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("foo", 42);
    let mut iter = m.into_iter();
    assert_eq!(42, iter.next().unwrap().1);
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn insert_and_iterate() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in m.iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_into_iterate() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in m.into_iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_gets() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    assert_eq!(16, *m.get("two").unwrap());
    Ok(())
}

#[test]
fn insert_and_gets_mut() -> Result<()> {
    let mut m: Map<i32, [i32; 3], 10> = Map::new();
    m.insert(42, [1, 2, 3]);
    let a = m.get_mut(42).unwrap();
    a[0] = 500;
    assert_eq!(500, m.get(42).unwrap()[0]);
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
struct Foo {
    v: Vec<u32>,
}

#[test]
fn insert_struct() -> Result<()> {
    let mut m: Map<u8, Foo, 8> = Map::new();
    let foo = Foo { v: vec![1, 2, 100] };
    m.insert(1, foo);
    assert_eq!(100, m.into_iter().next().unwrap().1.v[2]);
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
struct Composite {
    r: Map<u8, u8, 1>,
}

#[test]
fn insert_composite() -> Result<()> {
    let mut m: Map<u8, Composite, 8> = Map::new();
    let c = Composite { r: Map::new() };
    m.insert(1, c);
    assert_eq!(0, m.into_iter().next().unwrap().1.r.len());
    Ok(())
}

#[derive(Clone)]
struct Bar {}

#[test]
fn large_map_in_heap() -> Result<()> {
    let m: Box<Map<u64, [u64; 10], 10>> = Box::new(Map::new());
    assert_eq!(0, m.len());
    Ok(())
}
