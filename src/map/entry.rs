// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;
use core::mem;

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// Gets the given key’s corresponding entry in the map for in-place
    /// manipulation.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut letters: Map<_, _, 128> = Map::new();
    /// for ch in "a short treatise on fungi".chars() {
    ///     letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    /// }
    /// assert_eq!(letters[&'s'], 2);
    /// assert_eq!(letters[&'t'], 3);
    /// assert_eq!(letters[&'u'], 1);
    /// assert_eq!(letters.get(&'y'), None);
    /// ```
    pub fn entry(&mut self, k: K) -> Entry<'_, K, V, N> {
        if let Some((i, _)) = self.pairs[..self.len]
            .iter()
            .enumerate()
            .find(|(_, p)| unsafe { p.assume_init_ref() }.0 == k)
        {
            Entry::Occupied(OccupiedEntry {
                index: i,
                table: self,
            })
        } else {
            Entry::Vacant(VacantEntry {
                key: k,
                table: self,
            })
        }
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`][Map::entry] method on [`Map`].
pub enum Entry<'a, K, V, const N: usize> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, N>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, N>),
}

/// A view into an occupied entry in a `Map`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K, V, const N: usize> {
    index: usize,
    table: &'a mut Map<K, V, N>,
}

/// A view into a vacant entry in a `Map`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K, V, const N: usize> {
    key: K,
    table: &'a mut Map<K, V, N>,
}

impl<K, V, const N: usize> Entry<'_, K, V, N> {
    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    #[must_use]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    #[inline]
    #[allow(clippy::return_self_not_must_use)] // function has side effects (impure)
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K: PartialEq, V, const N: usize> Entry<'a, K, V, N> {
    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(3);
    /// assert_eq!(map["poneyland"], 3);
    /// *map.entry("poneyland").or_insert(10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<_, _, 3> = Map::new();
    /// let value = "hoho";
    /// map.entry("poneyland").or_insert_with(|| value);
    /// assert_eq!(map["poneyland"], "hoho");
    /// ```
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of
    /// the default function.
    ///
    /// This method allows for generating key-derived values
    /// for insertion by providing the default function a reference to the key
    /// that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the
    /// key is unnecessary, unlike with `.or_insert_with(|| ... )`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, usize, 3> = Map::new();
    /// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    /// assert_eq!(map["poneyland"], 9);
    /// ```
    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Sets the value of the entry, and returns an `OccupiedEntry`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, String, 3> = Map::new();
    /// let entry = map.entry("poneyland").insert_entry("hoho".to_string());
    /// assert_eq!(entry.key(), &"poneyland");
    /// ```
    #[inline]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, N> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }
}

impl<'a, K: PartialEq, V: Default, const N: usize> Entry<'a, K, V, N> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, Option<u32>, 3> = Map::new();
    /// map.entry("poneyland").or_default();
    /// assert_eq!(map["poneyland"], None);
    /// ```
    #[inline]
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<'a, K, V, const N: usize> OccupiedEntry<'a, K, V, N> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    #[must_use]
    pub fn key(&self) -> &K {
        unsafe { &self.table.item_ref(self.index).0 }
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in
    /// the entry with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see
    /// [`get_mut`][Self::get_mut].
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     *o.into_mut() += 10;
    /// }
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    #[inline]
    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        unsafe { self.table.value_mut(self.index) }
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.get(), &12);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self) -> &V {
        unsafe { &self.table.item_ref(self.index).1 }
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`][Self::into_mut].
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self) -> &mut V {
        unsafe { self.table.value_mut(self.index) }
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    #[inline]
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    /// Take the ownership of the key and value from the map.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     let _ = o.remove_entry();
    /// }
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    #[must_use = "if no need the return value, use `remove()` instead."]
    pub fn remove_entry(self) -> (K, V) {
        unsafe { self.table.remove_index_read(self.index) }
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// map.entry("poneyland").or_insert(12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    #[allow(clippy::must_use_candidate)]
    pub fn remove(self) -> V {
        unsafe { self.table.remove_index_read(self.index).1 }
    }
}

impl<K, V, const N: usize> VacantEntry<'_, K, V, N> {
    /// Gets a reference to the key that would be used when inserting a
    /// value through the `VacantEntry`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    #[must_use]
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     let _ = v.into_key();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_key(self) -> K {
        self.key
    }
}

impl<'a, K: PartialEq, V, const N: usize> VacantEntry<'a, K, V, N> {
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        let (index, _) = self.table.insert_ii(self.key, value, false);
        unsafe { self.table.value_mut(index) }
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns an `OccupiedEntry`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// use micromap::map::Entry;
    /// let mut map: Map<&str, u32, 3> = Map::new();
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     let _ = o.insert_entry(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[inline]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, N> {
        let (i, pair) = self.table.insert_ii(self.key, value, false);
        debug_assert!(pair.is_none());
        OccupiedEntry {
            index: i,
            table: self.table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;
    use super::Map;

    #[test]
    fn various() {
        let mut m: Map<char, u8, 10> =
            Map::from_iter([('a', 97), ('d', 100), ('c', 99), ('b', 98)]);
        let e: Entry<'_, char, u8, 10> = m.entry('c');
        assert_eq!(e.key(), &'c');
        m.entry('e').or_insert(b'e');
        m.entry('e').or_insert(b'e');
        assert_eq!(*m.entry('e').and_modify(|v| *v = 42).or_default(), 42);
        assert_eq!(m.entry('g').key(), &'g');
        assert_eq!(
            *m.entry('g').and_modify(|v| *v = 42).or_default(),
            u8::default()
        );
        let value = if let Entry::Occupied(mut entry) = m.entry('e') {
            let value = *entry.get();
            assert_eq!(value, 42);
            *entry.get_mut() = b'E';
            assert_eq!(*entry.get(), 69);
            let e = entry.into_mut();
            *e = b'e';
            value
        } else {
            100
        };
        assert_eq!(*m.entry('f').or_insert_with(|| value + 1), 43); // _ -> 43
        assert_eq!(*m.entry('f').or_insert_with(|| value + 2), 43); // no change
        assert_eq!(m.remove_entry(&'f'), Some(('f', 43))); // 43 -> _
        assert_eq!(
            *m.entry('f')
                .or_insert_with_key(|&key| key.try_into().unwrap()),
            102
        ); // _ -> 102
        assert_eq!(*m.entry('f').or_insert_with_key(|&_| 255), 102); // no change
        if let Entry::Occupied(entry) = m.entry('e') {
            assert_eq!(entry.remove(), 101);
        }
        if let Entry::Vacant(entry) = m.entry('e') {
            assert_eq!(entry.key(), &'e');
            assert_eq!(entry.into_key(), 'e');
        }
        if let Entry::Vacant(entry) = m.entry('e') {
            assert_eq!(entry.key(), &'e');
            entry.insert(b'e');
        }
        if let Entry::Occupied(mut entry) = m.entry('e') {
            entry.insert(b'E');
            assert_eq!(entry.remove_entry(), ('e', b'E'));
        }
        let occupied_entry = m.entry('e').insert_entry(b'e');
        assert_eq!(occupied_entry.get(), &b'e');
    }
}
