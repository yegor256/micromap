// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<K: Clone + PartialEq, V: Clone, const N: usize> Clone for Map<K, V, N> {
    fn clone(&self) -> Self {
        let mut m: Self = Self::new();
        for i in 0..self.len {
            m.item_write(i, self.item_ref(i).clone());
        }
        m.len = self.len;
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
