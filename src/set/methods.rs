// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

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

    /// Returns true if the set contains a value.
    #[inline]
    #[must_use]
    pub fn contains<Q: PartialEq + ?Sized>(&self, k: &Q) -> bool
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

/// Specialized methods available only on [`Set`].
impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let a = Set::from([1, 2, 3]);
    /// let mut b: Set<u32, 5> = Set::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint<const M: usize>(&self, other: &'_ Set<T, M>) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|v| !other.contains(v))
        } else {
            other.iter().all(|v| !self.contains(v))
        }
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let sup = Set::from([1, 2, 3]);
    /// let mut set: Set<u32, 5> = Set::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset<const M: usize>(&self, other: &'_ Set<T, M>) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|v| other.contains(v))
        } else {
            false
        }
    }

    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let sub = Set::from([1, 2]);
    /// let mut set: Set<u32, 5> = Set::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    #[inline]
    pub fn is_superset<const M: usize>(&self, other: &'_ Set<T, M>) -> bool {
        other.is_subset(self)
    }

    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given
    /// one. Returns the replaced value.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Set;
    ///
    /// let mut set: Set<_, 5> = Set::new();
    /// set.insert(Vec::<i32>::new());
    ///
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    /// set.replace(Vec::with_capacity(10));
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    /// ```
    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T> {
        let (_, existing_pair) = self.map.insert_i(value, ());
        existing_pair.map(|(k, ())| k)
    }
}
