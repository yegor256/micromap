// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use core::fmt::{self, Display, Formatter, Write};

impl<K: PartialEq + Display, V: Display, const N: usize> Display for Map<K, V, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first = true;
        f.write_char('{')?;
        for (k, v) in self {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            k.fmt(f)?;
            f.write_str(": ")?;
            v.fmt(f)?;
        }
        f.write_char('}')?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn displays_map() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        assert_eq!(r#"{one: 42, two: 16}"#, format!("{}", m));
    }
}
