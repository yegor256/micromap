// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! rkyv support for [`Map`], enabled by the `rkyv` feature.
//!
//! Unlike `serde`, `rkyv` performs zero-copy deserialization: the archived
//! bytes are read in place through [`ArchivedMap`], without constructing a
//! [`Map`] first. The [`Map`] itself stores its pairs in an array of
//! `MaybeUninit` slots, which cannot be reinterpreted in place; therefore a
//! dedicated archived representation is provided instead of an alias.
//!
//! ```
//! use micromap::map::rkyv::ArchivedMap;
//! use micromap::Map;
//! use rkyv::{access, rancor::Error, to_bytes};
//!
//! let mut map: Map<u8, u8, 8> = Map::new();
//! map.insert(1, 42);
//! let bytes = to_bytes::<Error>(&map).unwrap();
//! let archived = access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).unwrap();
//! assert_eq!(archived.len(), 1);
//! assert_eq!(archived.get(&1), Some(&42));
//! ```

use super::Map;
use core::{error::Error, fmt, mem::MaybeUninit, ptr};
use rkyv::{
    bytecheck::CheckBytes,
    munge::munge,
    option::ArchivedOption,
    primitive::ArchivedUsize,
    rancor::{fail, Fallible, Source},
    traits::{NoUndef, Portable},
    Archive, Deserialize, Place, Serialize,
};

/// The archived counterpart of a key-value pair of a [`Map`].
///
/// The pair has a fixed `#[repr(C)]` layout, which makes it safe to read
/// directly from serialized bytes.
#[derive(Debug)]
#[repr(C)]
pub struct ArchivedPair<K, V> {
    /// The archived key.
    pub key: K,
    /// The archived value.
    pub value: V,
}

/// The archived counterpart of [`Map`].
///
/// Unused capacity is stored as `None` slots, so the whole array is always
/// initialized and can be validated before access. Only the first
/// [`len()`][ArchivedMap::len] slots contain pairs.
#[repr(C)]
pub struct ArchivedMap<K, V, const N: usize> {
    /// The number of pairs stored in the map.
    len: ArchivedUsize,
    /// The archived pairs; the slots after `len` are `None`.
    pairs: [ArchivedOption<ArchivedPair<K, V>>; N],
}

/// Tag of an archived pair slot, mirroring the layout of the tag of
/// [`rkyv::option::ArchivedOption`].
#[repr(u8)]
enum ArchivedPairTag {
    /// The slot is unused.
    None,
    /// The slot holds a pair.
    Some,
}

// SAFETY: `ArchivedPairTag` is `repr(u8)` and so always consists of a single
// well-defined byte.
unsafe impl NoUndef for ArchivedPairTag {}

/// Mirror of the `None` variant of [`rkyv::option::ArchivedOption`], used to
/// resolve unused slots of the archived representation.
#[repr(C)]
struct ArchivedPairNone(ArchivedPairTag);

/// Mirror of the `Some` variant of [`rkyv::option::ArchivedOption`], used to
/// resolve pairs into the archived representation.
#[repr(C)]
struct ArchivedPairSome<K, V>(ArchivedPairTag, ArchivedPair<K, V>);

/// The resolver of a [`Map`].
#[doc(hidden)]
pub struct MapResolver<K, V, const N: usize> {
    /// Resolvers of the pairs; the slots after `len` are `None`.
    slots: [Option<(K, V)>; N],
}

impl<K, V, const N: usize> ArchivedMap<K, V, N> {
    /// Returns the number of pairs in the archived map.
    ///
    /// The reported length never exceeds the capacity `N`, even for archived
    /// maps obtained through unchecked APIs on corrupted data.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        usize::try_from(u64::from(self.len.to_native()))
            .unwrap_or(usize::MAX)
            .min(N)
    }

    /// Returns `true` if the archived map has no pairs.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// An iterator over the archived pairs in storage order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &ArchivedPair<K, V>> {
        self.pairs
            .iter()
            .take(self.len())
            .filter_map(ArchivedOption::as_ref)
    }
}

impl<K: PartialEq, V, const N: usize> ArchivedMap<K, V, N> {
    /// Returns the archived value paired with the given archived key.
    #[inline]
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.iter().find(|p| p.key == *key).map(|p| &p.value)
    }

    /// Returns `true` if the archived map contains the given archived key.
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

unsafe impl<K: Portable, V: Portable, const N: usize> Portable for ArchivedMap<K, V, N> {}

unsafe impl<K: Portable, V: Portable> Portable for ArchivedPair<K, V> {}

impl<K: Archive, V: Archive, const N: usize> Archive for Map<K, V, N> {
    type Archived = ArchivedMap<K::Archived, V::Archived, N>;
    type Resolver = MapResolver<K::Resolver, V::Resolver, N>;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(let ArchivedMap { len, pairs } = out);
        #[allow(clippy::cast_possible_truncation)] // `self.len` never exceeds `N`
        len.write(ArchivedUsize::from_native(self.len as _));
        let MapResolver { slots } = resolver;
        for (i, slot) in slots.into_iter().enumerate() {
            let out_i = unsafe {
                // SAFETY: `i` is less than `N`, because `slots` has exactly `N`
                // elements.
                pairs.index(i)
            };
            match slot {
                None => {
                    let out_none = unsafe {
                        // SAFETY: `ArchivedOption::None` and `ArchivedPairNone`
                        // have the same layout: a single `#[repr(u8)]` tag.
                        out_i.cast_unchecked::<ArchivedPairNone>()
                    };
                    munge!(let ArchivedPairNone(tag) = out_none);
                    tag.write(ArchivedPairTag::None);
                }
                Some((kr, vr)) => {
                    let (k, v) = unsafe {
                        // SAFETY: `serialize()` produces a resolver for every
                        // pair of the map and for nothing else, so `i` points
                        // to an initialized pair.
                        self.pairs[i].assume_init_ref()
                    };
                    let out_pair = unsafe {
                        // SAFETY: `ArchivedOption::Some` and `ArchivedPairSome`
                        // have the same layout: a `#[repr(u8)]` tag followed by
                        // a `#[repr(C)]` payload.
                        out_i.cast_unchecked::<ArchivedPairSome<K::Archived, V::Archived>>()
                    };
                    munge!(let ArchivedPairSome(tag, pair) = out_pair);
                    tag.write(ArchivedPairTag::Some);
                    munge!(let ArchivedPair { key, value } = pair);
                    k.resolve(kr, key);
                    v.resolve(vr, value);
                }
            }
        }
    }
}

impl<K, V, S, const N: usize> Serialize<S> for Map<K, V, N>
where
    K: Serialize<S>,
    V: Serialize<S>,
    S: Fallible + ?Sized,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let mut slots = MaybeUninit::<[Option<(K::Resolver, V::Resolver)>; N]>::uninit();
        let ptr = slots
            .as_mut_ptr()
            .cast::<Option<(K::Resolver, V::Resolver)>>();
        for (i, (k, v)) in self.iter().enumerate() {
            let resolver = (k.serialize(serializer)?, v.serialize(serializer)?);
            unsafe {
                // SAFETY: `i` is less than `self.len()`, which is at most `N`.
                ptr.add(i).write(Some(resolver));
            }
        }
        for i in self.len()..N {
            unsafe {
                // SAFETY: `i` is less than `N`.
                ptr.add(i).write(None);
            }
        }
        Ok(MapResolver {
            slots: unsafe {
                // SAFETY: all `N` slots have been initialized above; on an
                // error the function has already returned early.
                slots.assume_init()
            },
        })
    }
}

impl<K, V, D, const N: usize> Deserialize<Map<K, V, N>, D>
    for ArchivedMap<K::Archived, V::Archived, N>
where
    K: Archive,
    V: Archive,
    K::Archived: Deserialize<K, D>,
    V::Archived: Deserialize<V, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<Map<K, V, N>, D::Error> {
        let mut pairs: [MaybeUninit<(K, V)>; N] = [const { MaybeUninit::uninit() }; N];
        let mut count = 0;
        for slot in self.pairs.iter().take(self.len()) {
            match slot.as_ref() {
                Some(pair) => {
                    let key = pair.key.deserialize(deserializer)?;
                    let value = pair.value.deserialize(deserializer)?;
                    pairs[count].write((key, value));
                    count += 1;
                }
                None => break,
            }
        }
        Ok(Map { len: count, pairs })
    }
}

/// The error reported when an archived map claims more pairs than it can hold.
#[derive(Debug)]
struct CapacityExceeded {
    /// The length stored in the archived map.
    len: u64,
    /// The capacity of the archived map.
    capacity: usize,
}

impl fmt::Display for CapacityExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "archived map length {} exceeds its capacity {}",
            self.len, self.capacity
        )
    }
}

impl Error for CapacityExceeded {}

unsafe impl<K, V, C> CheckBytes<C> for ArchivedPair<K, V>
where
    K: CheckBytes<C>,
    V: CheckBytes<C>,
    C: Fallible + ?Sized,
{
    #[inline]
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
        K::check_bytes(unsafe { ptr::addr_of!((*value).key) }, context)?;
        V::check_bytes(unsafe { ptr::addr_of!((*value).value) }, context)?;
        Ok(())
    }
}

unsafe impl<K, V, C, const N: usize> CheckBytes<C> for ArchivedMap<K, V, N>
where
    K: CheckBytes<C>,
    V: CheckBytes<C>,
    C: Fallible + ?Sized,
    C::Error: Source,
{
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
        let len = unsafe { ptr::addr_of!((*value).len) };
        ArchivedUsize::check_bytes(len, context)?;
        let raw = u64::from(unsafe { (*len).to_native() });
        if raw > N as u64 {
            fail!(CapacityExceeded {
                len: raw,
                capacity: N,
            });
        }
        let pairs = unsafe { ptr::addr_of!((*value).pairs) };
        for i in 0..N {
            ArchivedOption::<ArchivedPair<K, V>>::check_bytes(
                unsafe { (*pairs).as_ptr().add(i) },
                context,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ArchivedMap, ArchivedPair};
    use crate::Map;
    use rkyv::{
        access, from_bytes, primitive::ArchivedU32, rancor::Error, string::ArchivedString, to_bytes,
    };

    #[test]
    fn round_trip_small_map() {
        let mut before: Map<u8, u8, 8> = Map::new();
        before.insert(1, 42);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 1);
        assert!(!archived.is_empty());
        assert_eq!(archived.get(&1), Some(&42));
        assert_eq!(archived.get(&2), None);
        assert!(archived.contains_key(&1));
        assert!(!archived.contains_key(&2));
        let after: Map<u8, u8, 8> = from_bytes::<Map<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(after.len(), before.len());
        assert_eq!(after.get(&1), Some(&42));
    }

    #[test]
    fn round_trip_empty_map() {
        let before: Map<u8, u8, 8> = Map::new();
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 0);
        assert!(archived.is_empty());
        let mut it = archived.iter();
        assert!(it.next().is_none());
        let after: Map<u8, u8, 8> = from_bytes::<Map<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert!(after.is_empty());
    }

    #[test]
    fn round_trip_full_map() {
        let mut before: Map<u8, u8, 8> = Map::new();
        for i in 0..8u8 {
            before.insert(i, i * 2);
        }
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 8);
        for pair in archived.iter() {
            let ArchivedPair { key, value } = pair;
            assert_eq!(*value, key.wrapping_mul(2));
        }
        let after: Map<u8, u8, 8> = from_bytes::<Map<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(after.len(), before.len());
        assert_eq!(after, before);
    }

    #[test]
    fn round_trip_heap_values() {
        let mut before: Map<u8, String, 4> = Map::new();
        before.insert(1, "one".to_string());
        before.insert(2, "two".to_string());
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<u8, ArchivedString, 4>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 2);
        let after: Map<u8, String, 4> =
            from_bytes::<Map<u8, String, 4>, Error>(&bytes[..]).unwrap();
        assert_eq!(after.get(&1).map(String::as_str), Some("one"));
        assert_eq!(after.get(&2).map(String::as_str), Some("two"));
    }

    #[test]
    fn access_wide_keys() {
        let mut before: Map<u32, u8, 4> = Map::new();
        before.insert(1, 10);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<ArchivedU32, u8, 4>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 1);
        let key = ArchivedU32::from_native(1);
        assert_eq!(archived.get(&key), Some(&10));
    }

    #[test]
    fn iteration_order_is_preserved() {
        let mut before: Map<u8, u8, 8> = Map::new();
        before.insert(5, 50);
        before.insert(3, 30);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).unwrap();
        let keys: Vec<u8> = archived.iter().map(|p| p.key).collect();
        let expected: Vec<u8> = before.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, expected);
    }

    #[test]
    fn rejects_corrupted_length() {
        let mut before: Map<u8, u8, 8> = Map::new();
        before.insert(1, 42);
        let mut bytes = to_bytes::<Error>(&before).unwrap();
        // The root `ArchivedMap` is stored at the very end of the buffer and
        // its `len` field comes first in the layout.
        let size = core::mem::size_of::<ArchivedMap<u8, u8, 8>>();
        let root = bytes.len() - size;
        let width = core::mem::size_of::<rkyv::primitive::ArchivedUsize>();
        let overflow = usize::MAX.to_le_bytes();
        bytes[root..root + width].copy_from_slice(&overflow[..width]);
        assert!(access::<ArchivedMap<u8, u8, 8>, Error>(&bytes[..]).is_err());
    }
}
