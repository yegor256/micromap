// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;
use core::borrow::Borrow;

impl<T, const N: usize> Set<T, N> {
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

    /// Remove all pairs from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        self.map.retain(|k, ()| f(k));
    }
}

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Returns `true` if the set contains a value.
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

    /// Adds a value to the set. (will not update)
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned, and the set is not
    ///   modified: original value is not replaced, and the value passed as argument is dropped.
    ///
    /// # Panics
    /// It may panic if there are too many pairs in the set already. Pay attention,
    /// it panics only in the "debug" mode. In the "release" mode, you are going to get
    /// undefined behavior. This is done for the sake of performance, in order to
    /// avoid a repetitive check for the boundary condition on every `insert()`.
    #[inline]
    pub fn insert(&mut self, k: T) -> bool {
        self.map.insert(k, ()).is_none()
    }

    /// Attempt to insert a value into the set. (will not update, and no panic)
    ///
    /// - If the value exists, whether the set is full or not, we update the
    ///   value (exclude key), and return `Some(Some(old_value))`;
    /// - If the key does not exist and the map is full already, we can do
    ///   nothing, so just return `None`;
    /// - If the key does not exist and the map has empty slot, we insert
    ///   into that slot and return `Some(None)`.
    ///
    ///
    /// - If the value existed, we do nothing, and return `Some(T)`, which is the value
    ///   parameter we passed in.
    /// - If the value does not exist and the set is full already, return `Some(T)` like
    ///   above.
    /// - If the value does not exist and the set has empty slot, we insert into that slot
    ///   and return `None`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// // Only the first element in the tuple is considered when determining whether
    /// // two `Foo`s are equal.
    /// #[derive(Debug)]
    /// struct Foo(u8, char);
    /// impl PartialEq for Foo {
    ///    fn eq(&self, other: &Self) -> bool {
    ///       self.0 == other.0
    ///     }
    /// }
    /// // Not necessary
    /// impl Eq for Foo {}
    /// // create Set for Foo
    /// let mut set: Set<Foo, 3> = Set::new();
    /// // For `Some(None)`, `Some(_)` indicates that the insertion was successful, `None`
    /// // in the former means that the inserted value was not in set before, that is, the
    /// // insertion occurred, otherwise, no-op was performed.
    /// assert_eq!(set.checked_insert(Foo(1, 'a')), None);
    /// let owned_foo = Foo(1, 'A'); // no `Clone` and `Copy` trait
    /// if let Some(getback_foo) = set.checked_insert(owned_foo) { // take the ownership
    ///     assert_eq!(getback_foo.1, 'A'); // get back the `owned_foo`
    /// } else {
    ///     unreachable!();
    /// }
    /// assert_eq!(set.checked_insert(Foo(2, 'b')), None);
    /// assert_eq!(set.checked_insert(Foo(3, 'c')), None);
    /// assert_eq!(set.len(), set.capacity());
    /// // map is full now.
    /// assert_eq!(set.checked_insert(Foo(2, 'B')).unwrap().1, 'B');
    /// // This insertion will cause capacity overflow, so no insertion is performed
    /// // and `None` is returned.
    /// assert_eq!(set.checked_insert(Foo(4, 'd')), None);
    /// ```
    #[inline]
    pub fn checked_insert(&mut self, k: T) -> Option<T> {
        if self.len() < N {
            self.map.insert_ii(k, (), false).1.map(|(k, ())| k)
        } else {
            self.map
                .insert_ii_for_full(k, (), false)
                .map(|(_, (k, ()))| k)
        }
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
    /// ```
    /// use micromap::Set;
    /// let a = Set::from([1, 2, 3]);
    /// let mut b: Set<u32, 5> = Set::new();
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
    /// ```
    /// use micromap::Set;
    /// let sup = Set::from([1, 2, 3]);
    /// let mut set: Set<u32, 5> = Set::new();
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
    /// ```
    /// use micromap::Set;
    /// let sub = Set::from([1, 2]);
    /// let mut set: Set<u32, 5> = Set::new();
    /// assert_eq!(set.is_superset(&sub), false);
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
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
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 5> = Set::new();
    /// set.insert(Vec::<i32>::new());
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    /// set.replace(Vec::with_capacity(10));
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    /// ```
    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T> {
        let (_, existing_pair) = self.map.insert_ii(value, (), true);
        existing_pair.map(|(k, ())| k)
    }
}
