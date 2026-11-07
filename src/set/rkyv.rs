// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! rkyv support for [`Set`], enabled by the `rkyv` feature.
//!
//! ```
//! use micromap::set::rkyv::ArchivedSet;
//! use micromap::Set;
//! use rkyv::{access, rancor::Error, to_bytes};
//!
//! let set: Set<u8, 8> = Set::from_iter([1, 2, 3]);
//! let bytes = to_bytes::<Error>(&set).unwrap();
//! let archived = access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).unwrap();
//! assert_eq!(archived.len(), 3);
//! assert!(archived.contains(&2));
//! ```

use super::Set;
use crate::map::{
    rkyv::{ArchivedMap, MapResolver},
    Map,
};
use rkyv::{
    bytecheck::CheckBytes,
    rancor::{Fallible, Source},
    traits::Portable,
    Archive, Deserialize, Place, Serialize,
};

/// The archived counterpart of [`Set`].
#[repr(transparent)]
pub struct ArchivedSet<T, const N: usize>(ArchivedMap<T, (), N>);

impl<T, const N: usize> ArchivedSet<T, N> {
    /// Returns the number of items in the archived set.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the archived set has no items.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// An iterator over the archived items in storage order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|p| &p.key)
    }
}

impl<T: PartialEq, const N: usize> ArchivedSet<T, N> {
    /// Returns `true` if the archived set contains the given archived item.
    #[inline]
    #[must_use]
    pub fn contains(&self, item: &T) -> bool {
        self.0.contains_key(item)
    }

    /// Returns the archived item equal to the given one, if present.
    ///
    /// The returned reference always points to the item stored in this
    /// archived set, never to the equal-looking item passed for lookup,
    /// which may carry a different payload for types whose [`PartialEq`]
    /// compares only part of the data.
    #[inline]
    #[must_use]
    pub fn get(&self, item: &T) -> Option<&T> {
        self.iter().find(|k| **k == *item)
    }
}

unsafe impl<T: Portable, const N: usize> Portable for ArchivedSet<T, N> {}

impl<T: Archive, const N: usize> Archive for Set<T, N> {
    type Archived = ArchivedSet<T::Archived, N>;
    type Resolver = MapResolver<T::Resolver, <() as Archive>::Resolver, N>;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        unsafe {
            // SAFETY: `Set` is `#[repr(transparent)]` over `Map<T, (), N>`, and
            // `ArchivedSet` is `#[repr(transparent)]` over the corresponding
            // archived map, so both places have the same layout.
            self.map.resolve(
                resolver,
                out.cast_unchecked::<<Map<T, (), N> as Archive>::Archived>(),
            );
        }
    }
}

impl<T, S, const N: usize> Serialize<S> for Set<T, N>
where
    T: Serialize<S>,
    S: Fallible + ?Sized,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.map.serialize(serializer)
    }
}

impl<T, D, const N: usize> Deserialize<Set<T, N>, D> for ArchivedSet<T::Archived, N>
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<Set<T, N>, D::Error> {
        let map = <ArchivedMap<T::Archived, (), N> as Deserialize<Map<T, (), N>, D>>::deserialize(
            &self.0,
            deserializer,
        )?;
        Ok(Set { map })
    }
}

unsafe impl<T, C, const N: usize> CheckBytes<C> for ArchivedSet<T, N>
where
    T: CheckBytes<C>,
    C: Fallible + ?Sized,
    C::Error: Source,
{
    #[inline]
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
        ArchivedMap::<T, (), N>::check_bytes(value.cast::<ArchivedMap<T, (), N>>(), context)
    }
}

#[cfg(test)]
mod tests {
    use super::ArchivedSet;
    use crate::Set;
    use rkyv::{
        access,
        bytecheck::CheckBytes,
        from_bytes,
        munge::munge,
        primitive::{ArchivedI32, ArchivedU64},
        rancor::{Error, Fallible},
        to_bytes,
        traits::Portable,
        Archive, Place, Serialize,
    };

    /// An item whose [`PartialEq`] compares only its id, ignoring its
    /// payload, so that lookup semantics can be checked: the set must return
    /// the stored item, which may carry a different payload than the equal
    /// item passed for lookup.
    #[derive(Debug)]
    struct Item {
        /// Compared by [`PartialEq`].
        id: u8,
        /// Ignored by [`PartialEq`].
        payload: u64,
    }

    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    /// The archived counterpart of [`Item`].
    #[derive(Debug)]
    #[repr(C)]
    struct ArchivedItem {
        /// Compared by [`PartialEq`].
        id: u8,
        /// Ignored by [`PartialEq`].
        payload: ArchivedU64,
    }

    impl PartialEq for ArchivedItem {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    // SAFETY: `ArchivedItem` consists of a portable primitive and a `u8` in
    // a `#[repr(C)]` layout.
    unsafe impl Portable for ArchivedItem {}

    unsafe impl<C: Fallible + ?Sized> CheckBytes<C> for ArchivedItem {
        // SAFETY: both fields are primitive types, whose every bit pattern
        // is valid, so there is nothing to check.
        unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), C::Error> {
            Ok(())
        }
    }

    /// The resolver of an [`Item`].
    struct ItemResolver;

    impl Archive for Item {
        type Archived = ArchivedItem;
        type Resolver = ItemResolver;

        fn resolve(&self, _: ItemResolver, out: Place<ArchivedItem>) {
            munge!(let ArchivedItem { id, payload } = out);
            id.write(self.id);
            payload.write(ArchivedU64::from_native(self.payload));
        }
    }

    impl<S: Fallible + ?Sized> Serialize<S> for Item {
        fn serialize(&self, _: &mut S) -> Result<ItemResolver, S::Error> {
            Ok(ItemResolver)
        }
    }

    #[test]
    fn round_trip_small_set() {
        let before: Set<u8, 8> = Set::from_iter([1, 2, 3]);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 3);
        assert!(!archived.is_empty());
        assert!(archived.contains(&2));
        assert!(!archived.contains(&4));
        assert_eq!(archived.get(&3), Some(&3));
        assert_eq!(archived.get(&4), None);
        let after: Set<u8, 8> = from_bytes::<Set<u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(after, before);
    }

    #[test]
    fn access_wide_items() {
        let before: Set<i32, 8> = Set::from_iter([1, 2, 3]);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<ArchivedI32, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 3);
        let two = ArchivedI32::from_native(2);
        assert!(archived.contains(&two));
    }

    #[test]
    fn round_trip_empty_set() {
        let before: Set<u8, 8> = Set::new();
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 0);
        assert!(archived.is_empty());
        let after: Set<u8, 8> = from_bytes::<Set<u8, 8>, Error>(&bytes[..]).unwrap();
        assert!(after.is_empty());
    }

    #[test]
    fn round_trip_full_set() {
        let before: Set<u8, 8> = Set::from([0, 1, 2, 3, 4, 5, 6, 7]);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(archived.len(), 8);
        assert_eq!(archived.iter().count(), 8);
        let after: Set<u8, 8> = from_bytes::<Set<u8, 8>, Error>(&bytes[..]).unwrap();
        assert_eq!(after, before);
    }

    #[test]
    fn rejects_corrupted_length() {
        let before: Set<u8, 8> = Set::from_iter([1, 2]);
        let mut bytes = to_bytes::<Error>(&before).unwrap();
        let size = core::mem::size_of::<ArchivedSet<u8, 8>>();
        let root = bytes.len() - size;
        let width = core::mem::size_of::<rkyv::primitive::ArchivedUsize>();
        let overflow = usize::MAX.to_le_bytes();
        bytes[root..root + width].copy_from_slice(&overflow[..width]);
        assert!(access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).is_err());
    }

    #[test]
    fn get_returns_pointer_to_the_stored_element() {
        let before: Set<u8, 8> = Set::from_iter([1, 2, 3]);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<u8, 8>, Error>(&bytes[..]).unwrap();
        let needle = 2_u8;
        let got = archived.get(&needle).unwrap();
        // The lookup argument itself must not be returned by reference.
        assert_ne!(core::ptr::from_ref(got), core::ptr::from_ref(&needle));
        // The stored element must be returned instead.
        let stored = archived.iter().find(|k| **k == needle).unwrap();
        assert!(core::ptr::eq(got, stored));
    }

    #[test]
    fn get_returns_the_stored_item_not_the_equal_lookup_one() {
        let before: Set<Item, 4> = Set::from_iter([
            Item {
                id: 1,
                payload: 100,
            },
            Item {
                id: 2,
                payload: 200,
            },
        ]);
        let bytes = to_bytes::<Error>(&before).unwrap();
        let archived = access::<ArchivedSet<ArchivedItem, 4>, Error>(&bytes[..]).unwrap();
        let lookup = ArchivedItem {
            id: 2,
            payload: ArchivedU64::from_native(999),
        };
        let got = archived.get(&lookup).unwrap();
        // Equal to the lookup item...
        assert_eq!(got, &lookup);
        // ...but carrying the payload of the stored one.
        assert_eq!(got.payload.to_native(), 200);
        assert_ne!(got.payload.to_native(), lookup.payload.to_native());
        // ...exactly the stored element.
        assert!(core::ptr::eq(got, archived.iter().nth(1).unwrap()));
    }
}
