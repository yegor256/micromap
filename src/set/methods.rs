// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;
use core::borrow::Borrow;

impl<T, const N: usize> Set<T, N> {
    /// Returns the number of elements the set can hold.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let set: Set<i32, 100> = Set::new();
    /// assert_eq!(set.capacity(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// assert!(set.is_empty());
    /// set.insert(1);
    /// assert!(!set.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(1);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.map.len()
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// set.insert(1);
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set = Set::from([1, 2, 3, 4, 5, 6]);
    /// set.retain(|&k| k % 2 == 0);
    /// assert_eq!(set, Set::from([2, 4, 6]));
    /// ```
    ///
    /// # Performance
    /// In the current implementation, this operation takes O(len) time.
    #[inline]
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        self.map.retain(|k, ()| f(k));
    }
}

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`PartialEq`] on the borrowed form *must* match those for the value
    /// type.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let set = Set::from([1, 2, 3]);
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, k: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        self.map.contains_key(k)
    }

    /// Removes a value from the set. Returns whether the value was present
    /// in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`PartialEq`] on the borrowed form *must* match those for the value
    /// type.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// set.insert(2);
    /// assert_eq!(set.remove(&2), true);
    /// assert_eq!(set.remove(&2), false);
    /// ```
    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: PartialEq + ?Sized,
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
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    ///
    /// # Panics
    /// It may panic if there are too many items in the set already to contain another new item.
    #[inline]
    pub fn insert(&mut self, k: T) -> bool {
        self.map.insert(k, ()).is_none()
    }

    /// Attempt to insert a value into the set. (will not update, and no panic)
    ///
    /// - If the value exists, whether the set is full or not, we update the
    ///   value (exclude key), and return `Some(Some(old_value))`;
    /// - If the key does not exist and the set is full already, we can do
    ///   nothing, so just return `None`;
    /// - If the key does not exist and the set has empty slot, we insert
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
    /// // set is full now.
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

    /// Insert a value into the set without bound check in release mode. (with panic
    /// and undefined behavior possible)
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned, and the set is not
    ///   modified: original value is not replaced, and the value passed as argument is dropped.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// assert_eq!(set.len(), 0);
    /// assert_eq!(set.capacity(), 3);
    /// unsafe {
    ///     assert_eq!(set.insert_unchecked(0), true);
    ///     assert_eq!(set.insert_unchecked(1), true);
    ///     assert_eq!(set.insert_unchecked(2), true);
    ///     assert_eq!(set.insert_unchecked(2), false);
    ///     assert_eq!(set.len(), set.capacity()); // 3
    ///     // assert_eq!(set.insert_unchecked(3), true); // CAN NOT DO THIS!
    /// }
    /// ```
    ///
    /// # Panics
    /// It may panic if there are too many items in the set already. Pay attention,
    /// it panics only in the `debug` mode. In the `release` mode, you are going to get
    /// **undefined behavior**. This is done for the sake of performance, in order to
    /// avoid a repetitive check for the boundary condition on every `insert()`.
    ///
    /// # Safety
    /// Calling this method to add a new key-value pair when the [`Set`] is already
    /// full is **undefined behavior instead of panic**. So you need to make sure that
    /// the set is not full before calling.
    #[inline]
    pub unsafe fn insert_unchecked(&mut self, k: T) -> bool {
        self.map.insert_unchecked(k, ()).is_none()
    }

    /// Returns a reference to the value in the set, if any, that is equal
    /// to the given value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`PartialEq`] on the borrowed form *must* match those for the value
    /// type.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let set = Set::from([1, 2, 3]);
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, k: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        self.map.get_key_value(k).map(|p| p.0)
    }

    /// Removes and returns the value in the set, if any, that is equal to the
    /// given one.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`PartialEq`] on the borrowed form *must* match those for the value
    /// type.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set = Set::from([1, 2, 3]);
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    #[inline]
    #[must_use = "if no need the return value, use `remove()` instead."]
    pub fn take<Q>(&mut self, k: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: PartialEq + ?Sized,
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
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
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
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn test_capacity() {
        let set: Set<i32, 10> = Set::new();
        assert_eq!(set.capacity(), 10);
    }

    #[test]
    fn test_is_empty() {
        let mut set: Set<i32, 5> = Set::new();
        assert!(set.is_empty());
        set.insert(1);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_len() {
        let mut set: Set<i32, 5> = Set::new();
        assert_eq!(set.len(), 0);
        set.insert(1);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut set: Set<i32, 5> = Set::new();
        set.insert(1);
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_retain() {
        let mut set = Set::from([1, 2, 3, 4, 5]);
        set.retain(|&x| x % 2 == 0);
        assert_eq!(set, Set::from([2, 4]));
    }

    #[test]
    fn test_contains() {
        let set = Set::from([1, 2, 3]);
        assert!(set.contains(&1));
        assert!(!set.contains(&4));
    }

    #[test]
    fn test_remove() {
        let mut set: Set<i32, 5> = Set::new();
        set.insert(2);
        assert!(set.remove(&2));
        assert!(!set.remove(&2));
    }

    #[test]
    fn test_insert() {
        let mut set: Set<i32, 3> = Set::new();
        assert!(set.insert(2));
        assert!(!set.insert(2));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_checked_insert() {
        let mut set: Set<i32, 2> = Set::new();
        assert_eq!(set.checked_insert(1), None);
        assert_eq!(set.checked_insert(2), None);
        assert_eq!(set.checked_insert(3), None);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_insert_unchecked() {
        let mut set: Set<i32, 3> = Set::new();
        unsafe {
            assert!(set.insert_unchecked(1));
            assert!(set.insert_unchecked(2));
            assert!(!set.insert_unchecked(2));
        }
    }

    #[test]
    fn test_get() {
        let set = Set::from([1, 2, 3]);
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&4), None);
    }

    #[test]
    fn test_take() {
        let mut set = Set::from([1, 2, 3]);
        assert_eq!(set.take(&2), Some(2));
        assert_eq!(set.take(&2), None);
    }

    #[test]
    fn test_is_disjoint() {
        let a = Set::from([1, 2, 3]);
        let mut b: Set<u32, 5> = Set::new();
        assert!(a.is_disjoint(&b));
        assert!(b.is_disjoint(&a));
        b.insert(4);
        assert!(a.is_disjoint(&b));
        assert!(b.is_disjoint(&a));
        b.insert(1);
        assert!(!a.is_disjoint(&b));
        assert!(!b.is_disjoint(&a));
    }

    #[test]
    fn test_is_subset() {
        let sup = Set::from([1, 2, 3]);
        let mut set: Set<u32, 5> = Set::new();
        assert!(set.is_subset(&sup));
        set.insert(2);
        assert!(set.is_subset(&sup));
        set.insert(4);
        assert!(!set.is_subset(&sup));
    }

    #[test]
    fn test_is_superset() {
        let sub = Set::from([1, 2]);
        let mut set: Set<u32, 5> = Set::new();
        assert!(!set.is_superset(&sub));
        set.insert(1);
        set.insert(2);
        assert!(set.is_superset(&sub));
    }

    #[test]
    fn test_replace() {
        let mut set: Set<Vec<i32>, 5> = Set::new();
        set.insert(Vec::new());
        assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
        set.replace(Vec::with_capacity(10));
        assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    }
}
