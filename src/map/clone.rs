// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;

impl<K, V, const N: usize> Clone for Map<K, V, N>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let mut m = Self::new();
        for (i, src) in self.pairs[..self.len].iter().enumerate() {
            // SAFETY: only slots below `self.len` are initialized in a valid map.
            let pair = unsafe { src.assume_init_ref() }.clone();
            m.pairs[i].write(pair);
            // The length is advanced only after the pair is fully written, so
            // a panic in `Clone::clone()` above can't make the `Drop` impl of
            // the map call `assume_init_drop()` on uninitialized slots during
            // unwinding.
            m.len = i + 1;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn map_can_be_cloned() {
        let mut m: Map<u8, u8, 16> = Map::new();
        m.insert(0, 42);
        assert_eq!(42, *m.clone().get(&0).unwrap());
    }

    #[test]
    fn empty_map_can_be_cloned() {
        let m: Map<u8, u8, 0> = Map::new();
        assert!(m.clone().is_empty());
    }

    /// A value that counts its drops and may panic in its `Clone` impl,
    /// to check that a panicking `Clone` can't make the partially cloned
    /// map drop uninitialized slots during unwinding.
    #[cfg(test)]
    struct CloneBomb {
        drops: Arc<AtomicUsize>,
        clones: Arc<AtomicUsize>,
        armed: bool,
    }

    impl Clone for CloneBomb {
        fn clone(&self) -> Self {
            self.clones.fetch_add(1, Ordering::SeqCst);
            assert!(!self.armed, "CloneBomb exploded while cloning");
            CloneBomb {
                drops: Arc::clone(&self.drops),
                clones: Arc::clone(&self.clones),
                armed: self.armed,
            }
        }
    }

    impl Drop for CloneBomb {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn clone_keeps_every_pair_and_drops_each_exactly_once() {
        let drops = Arc::new(AtomicUsize::new(0));
        let clones = Arc::new(AtomicUsize::new(0));
        let mut m: Map<u8, CloneBomb, 4> = Map::new();
        for i in 0..3 {
            m.insert(
                i,
                CloneBomb {
                    drops: Arc::clone(&drops),
                    clones: Arc::clone(&clones),
                    armed: false,
                },
            );
        }
        let c = m.clone();
        assert_eq!(clones.load(Ordering::SeqCst), 3);
        assert_eq!(c.len(), 3);
        drop(c);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
        drop(m);
        assert_eq!(drops.load(Ordering::SeqCst), 6);
    }

    #[test]
    fn clone_does_not_drop_uninitialized_slots_when_a_clone_panics_immediately() {
        let drops = Arc::new(AtomicUsize::new(0));
        let clones = Arc::new(AtomicUsize::new(0));
        let mut m: Map<u8, CloneBomb, 4> = Map::new();
        for i in 0..2 {
            m.insert(
                i,
                CloneBomb {
                    drops: Arc::clone(&drops),
                    clones: Arc::clone(&clones),
                    armed: i == 0,
                },
            );
        }
        let caught = catch_unwind(AssertUnwindSafe(|| m.clone()));
        assert!(caught.is_err(), "clone() must panic in the armed Clone");
        assert_eq!(clones.load(Ordering::SeqCst), 1);
        assert_eq!(
            drops.load(Ordering::SeqCst),
            0,
            "the partially cloned map must not drop uninitialized slots"
        );
        drop(m);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn clone_does_not_drop_uninitialized_slots_when_a_clone_panics_after_a_prefix() {
        let drops = Arc::new(AtomicUsize::new(0));
        let clones = Arc::new(AtomicUsize::new(0));
        let mut m: Map<u8, CloneBomb, 4> = Map::new();
        for i in 0..3 {
            m.insert(
                i,
                CloneBomb {
                    drops: Arc::clone(&drops),
                    clones: Arc::clone(&clones),
                    armed: i == 1,
                },
            );
        }
        let caught = catch_unwind(AssertUnwindSafe(|| m.clone()));
        assert!(caught.is_err(), "clone() must panic in the armed Clone");
        assert_eq!(clones.load(Ordering::SeqCst), 2);
        assert_eq!(
            drops.load(Ordering::SeqCst),
            1,
            "only the successfully cloned prefix may be dropped, not uninitialized slots"
        );
        drop(m);
        assert_eq!(drops.load(Ordering::SeqCst), 4);
    }
}
