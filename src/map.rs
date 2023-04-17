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
use crate::{IntoIter, Iter, Map, Pair};
use std::mem::MaybeUninit;

impl<K: Copy + PartialEq, V: Clone + Copy, const N: usize> Default for Map<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Copy + PartialEq, V: Clone + Copy, const N: usize> Map<K, V, N> {
    /// Make it.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            next: 0,
            pairs: unsafe { *MaybeUninit::<[Pair<K, V>; N]>::uninit().as_mut_ptr() },
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<K, V, N> {
        Iter {
            next: self.next,
            pos: 0,
            pairs: &self.pairs,
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<K, V, N> {
        IntoIter {
            next: self.next,
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
        if self.next == 0 {
            return 0;
        }
        let mut busy = 0;
        for i in 0..N {
            if self.next <= i {
                break;
            }
            if self.pairs[i].is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Does the map contain this key?
    #[inline]
    pub fn contains_key(&self, k: K) -> bool {
        for i in 0..N {
            if self.next <= i {
                break;
            }
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
            if self.next <= i {
                break;
            }
            if let Present((bk, _bv)) = &self.pairs[i] {
                if *bk == k {
                    self.pairs[i] = Absent;
                    break;
                }
            }
        }
    }

    /// Insert a single pair into the map.
    ///
    /// # Panics
    ///
    /// It may panic if there are too many pairs in the map already.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        self.remove(k);
        for i in 0..N {
            if self.next <= i {
                break;
            }
            if !self.pairs[i].is_some() {
                self.pairs[i] = Present((k, v));
                return;
            }
        }
        if self.next < N {
            self.pairs[self.next] = Present((k, v));
            self.next += 1;
            return;
        }
        panic!("There are only {N} pairs available in the map and all of them are already occupied")
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    pub fn get(&self, k: K) -> Option<&V> {
        for i in 0..N {
            if self.next <= i {
                break;
            }
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
            if self.next <= i {
                break;
            }
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

#[test]
fn checks_key() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    assert!(m.contains_key("one"));
    assert!(!m.contains_key("another"));
    Ok(())
}

#[test]
fn gets_missing_key() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    assert!(m.get("two").is_none());
    Ok(())
}

#[test]
fn mut_gets_missing_key() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    assert!(m.get_mut("two").is_none());
    Ok(())
}

#[test]
fn removes_simple_pair() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.remove("one");
    m.remove("another");
    assert!(m.get("one").is_none());
    Ok(())
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Foo {
    v: [u32; 3],
}

#[test]
fn insert_struct() -> Result<()> {
    let mut m: Map<u8, Foo, 8> = Map::new();
    let foo = Foo { v: [1, 2, 100] };
    m.insert(1, foo);
    assert_eq!(100, m.into_iter().next().unwrap().1.v[2]);
    Ok(())
}

#[cfg(test)]
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Bar {}

#[test]
fn large_map_in_heap() -> Result<()> {
    let m: Box<Map<u64, [u64; 10], 10>> = Box::new(Map::new());
    assert_eq!(0, m.len());
    Ok(())
}
