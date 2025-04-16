// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;
use core::mem;

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    pub fn entry(&mut self, k: K) -> Entry<'_, K, V, N> {
        for i in 0..self.len {
            let p = unsafe { self.item_ref(i) };
            if p.0 == k {
                return Entry::Occupied(OccupiedEntry {
                    index: i,
                    table: self,
                });
            }
        }
        Entry::Vacant(VacantEntry {
            key: k,
            table: self,
        })
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`Map`].
///
/// [`entry`]: Map::entry
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
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    #[must_use]
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
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }
}

impl<'a, K: PartialEq, V: Default, const N: usize> Entry<'a, K, V, N> {
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<'a, K, V, const N: usize> OccupiedEntry<'a, K, V, N> {
    #[must_use]
    pub fn key(&self) -> &K {
        unsafe { &self.table.item_ref(self.index).0 }
    }

    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        unsafe { self.table.value_mut(self.index) }
    }

    #[must_use]
    pub fn get(&self) -> &V {
        unsafe { &self.table.item_ref(self.index).1 }
    }

    pub fn get_mut(&mut self) -> &mut V {
        unsafe { self.table.value_mut(self.index) }
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    #[must_use]
    pub fn remove_entry(self) -> (K, V) {
        unsafe { self.table.remove_index_read(self.index) }
    }

    #[must_use]
    pub fn remove(self) -> V {
        unsafe { self.table.remove_index_read(self.index).1 }
    }
}

impl<K, V, const N: usize> VacantEntry<'_, K, V, N> {
    pub const fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }
}

impl<'a, K: PartialEq, V, const N: usize> VacantEntry<'a, K, V, N> {
    pub fn insert(self, value: V) -> &'a mut V {
        let (index, _) = self.table.insert_ii(self.key, value, false);
        unsafe { self.table.value_mut(index) }
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;
    use crate::Map;

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
    }
}
