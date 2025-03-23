// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

mod clone;
mod ctors;
mod debug;
mod display;
mod drain;
mod eq;
mod from;
mod iterators;
mod methods;
#[cfg(feature = "serde")]
mod serialization;

use crate::Map;

pub mod difference;
pub mod intersection;
// pub mod symmetric_difference;
// pub mod union;

/// A faster alternative of [`std::collections::HashSet`].
///
/// For example, this is how you make a set, which is allocated on stack and is capable of storing
/// up to eight key-values pairs:
///
/// ```
/// let mut m : micromap::Set<u64, 8> = micromap::Set::new();
/// m.insert(1);
/// m.insert(2);
/// # #[cfg(std)]
/// assert_eq!(2, m.len());
/// ```
///
/// It is faster because it doesn't use a hash function at all. It simply keeps
/// all pairs in an array and when it's necessary to find a value, it goes through
/// all pairs comparing the needle with each pair available. Also it is faster
/// because it doesn't use heap. When a [`Set`] is being created, it allocates the necessary
/// space on stack. That's why the maximum size of the set must be provided in
/// compile time.
///
/// It is also faster because it doesn't grow in size. When a [`Set`] is created,
/// its size is fixed on stack. If an attempt is made to insert too many keys
/// into it, it simply panics. Moreover, in the "release" mode it doesn't panic,
/// but its behaviour is undefined. In the "release" mode all boundary checks
/// are disabled, for the sake of higher performance.
#[repr(transparent)]
pub struct Set<T: PartialEq, const N: usize> {
    map: Map<T, (), N>,
}

/// Iterator over the [`Set`].
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetIter<'a, T> {
    iter: crate::Keys<'a, T, ()>,
}

/// Into-iterator over the [`Set`].
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetIntoIter<T: PartialEq, const N: usize> {
    iter: crate::IntoKeys<T, (), N>,
}

#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetDrain<'a, T: PartialEq> {
    iter: crate::Drain<'a, T, ()>,
}

#[cfg(test)]
mod tests {
    use crate::Set;

    #[test]
    fn various() {
        let mut set: Set<i32, 10> = Set::new();
        let set_default: Set<i32, 10> = Set::default();
        assert!(set == set_default);

        assert_eq!(set.capacity(), 10);
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
        set.insert(0);
        assert_eq!(set.len(), 1);
        set.insert(0);
        assert_eq!(set.len(), 1);
        set.insert(1);
        assert_eq!(set.len(), 2);

        let mut drain = set.drain();
        assert_eq!(drain.len(), 2);
        assert!(drain.next().is_some());
        assert_eq!(drain.len(), 1);
        assert_eq!(drain.len(), drain.size_hint().0);
        assert_eq!(drain.len(), drain.size_hint().1.unwrap());
        drop(drain);

        set.clear();

        for n in 0..10 {
            set.insert(n);
        }
        assert_eq!(set.len(), 10);
        assert!(set.contains(&0));
        assert_eq!(set.take(&0), Some(0));
        assert!(!set.contains(&10));

        assert_eq!(set.remove(&0), false);
        set.insert(0);
        assert_eq!(set.remove(&0), true);
        set.insert(10);

        assert_eq!(set.get(&10), Some(&10));
        assert_eq!(set.get(&11), None);

        set.retain(|&k| k % 2 == 0);

        let mut it = set.iter();
        assert!(it.next().is_some());
        assert_eq!(it.len(), 4);
        assert_eq!(it.len(), it.size_hint().0);
        assert_eq!(it.len(), it.size_hint().1.unwrap());

        let mut it_into = set.into_iter();
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_some());
        assert_eq!(it_into.len(), 2);
        assert_eq!(it_into.len(), it_into.size_hint().0);
        assert_eq!(it_into.len(), it_into.size_hint().1.unwrap());
    }

    #[test]
    fn test_set_from() {
        let set_a = Set::from(['a', 'b', 'c', 'd']);
        let set_b: Set<_, 6> = Set::from_iter(['a', 'a', 'd', 'b', 'a', 'd', 'c', 'd', 'c']);
        let set_c = set_a.clone();
        assert_eq!(set_a, set_b);
        assert_eq!(set_a, set_c);
    }
}
