// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use core::fmt;
use core::fmt::Write;

impl<K, V, const N: usize> fmt::Display for Map<K, V, N>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('{')?;
        let mut it = self.iter();
        if let Some((k, v)) = it.next() {
            write!(f, "{k}: {v}")?;
            it.try_for_each(|(k, v)| write!(f, ", {k}: {v}"))?;
        }
        f.write_char('}')?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_empty_map() {
        let m: Map<String, i32, 0> = Map::new();
        assert_eq!(r#"{}"#, format!("{}", m));
    }

    #[test]
    fn displays_one_item_map() {
        let mut m: Map<u32, bool, 1> = Map::new();
        m.insert(42, true);
        assert_eq!(r#"{42: true}"#, format!("{}", m));
    }

    #[test]
    fn displays_map() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        assert_eq!(r#"{one: 42, two: 16}"#, format!("{}", m));
    }
}
