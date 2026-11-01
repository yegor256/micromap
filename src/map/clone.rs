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
        m.len = self.len;
        m.pairs
            .iter_mut()
            .zip(self.pairs[..self.len].iter())
            .for_each(|(dst, src)| unsafe {
                dst.write(src.assume_init_ref().clone());
            });
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
