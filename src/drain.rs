// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Drain;
use core::iter::FusedIterator;

impl<K, V> Drop for Drain<'_, K, V> {
    fn drop(&mut self) {
        for pair in &mut self.iter {
            unsafe { pair.assume_init_drop() };
        }
    }
}

impl<K: PartialEq, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| unsafe { p.assume_init_read() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<K: PartialEq, V> ExactSizeIterator for Drain<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: PartialEq, V> FusedIterator for Drain<'_, K, V> {}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn normal_drain() {
        let mut map = Map::<char, u8, 10>::from_iter([('a', 97), ('b', 98), ('c', 99), ('d', 100)]);
        let mut cloned_map = map.clone();

        let mut drain = map.drain();

        // For ExactSizeIterator
        assert_eq!(drain.len(), drain.size_hint().0);

        // Consume the first two items by iterator
        assert_eq!(drain.next(), Some(('a', 97)));
        assert_eq!(drain.next(), Some(('b', 98)));

        // We can fuse the drain
        let mut fuse_it = drain.fuse();
        assert_eq!(fuse_it.next(), Some(('c', 99)));
        assert_eq!(fuse_it.next(), Some(('d', 100)));

        // Further calls to next() should return None
        assert!(fuse_it.next().is_none());
        // Then fuse works. (It doesn't make sense in our Drain really, but it can.)
        assert!(fuse_it.next().is_none());

        let mut drain = cloned_map.drain();
        assert_eq!(drain.next(), Some(('a', 97)));
        // Three elements left for Drop
        drop(drain);
    }
}
