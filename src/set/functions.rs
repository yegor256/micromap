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

use crate::{Set, SetDrain};
use core::borrow::Borrow;

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Get its total capacity.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Is it empty?
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Return the total number of pairs inside.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.map.len()
    }

    /// Clears the set, returning all elements as an iterator. Keeps the allocated memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it drops the remaining elements. The returned iterator keeps a mutable borrow on the set to optimize its implementation.
    pub fn drain(&mut self) -> SetDrain<'_, T> {
        SetDrain {
            iter: self.map.drain(),
        }
    }

    /// Does the set contain this key?
    #[inline]
    #[must_use]
    pub fn contains_key<Q: PartialEq + ?Sized>(&self, k: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        self.map.contains_key(k)
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    #[inline]
    pub fn remove<Q: PartialEq + ?Sized>(&mut self, k: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        self.map.remove(k).is_some()
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// If the set did not previously contain this value, true is returned.
    /// If the set already contained this value, false is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.
    ///
    /// # Panics
    ///
    /// It may panic if there are too many pairs in the set already. Pay attention,
    /// it panics only in the "debug" mode. In the "release" mode, you are going to get
    /// undefined behavior. This is done for the sake of performance, in order to
    /// avoid a repetitive check for the boundary condition on every `insert()`.
    #[inline]
    pub fn insert(&mut self, k: T) -> bool {
        self.map.insert(k, ()).is_none()
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    pub fn get<Q: PartialEq + ?Sized>(&self, k: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
    {
        self.map.get_key_value(k).map(|p| p.0)
    }

    /// Remove all pairs from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F: Fn(&T) -> bool>(&mut self, f: F) {
        self.map.retain(|k, ()| f(k));
    }

    /// Removes a key from the set, returning the stored key and value if the
    /// key was previously in the set.
    #[inline]
    pub fn take<Q: PartialEq + ?Sized>(&mut self, k: &Q) -> Option<T>
    where
        T: Borrow<Q>,
    {
        self.map.remove_entry(k).map(|p| p.0)
    }
}
