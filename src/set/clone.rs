// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;

impl<T: Clone, const N: usize> Clone for Set<T, N> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}
