// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Set;
use core::fmt::{self, Display, Formatter, Write};

impl<T: PartialEq + Display, const N: usize> Display for Set<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first = true;
        f.write_char('{')?;
        for k in self {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            k.fmt(f)?;
        }
        f.write_char('}')?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn displays_set() {
        let mut m: Set<String, 10> = Set::new();
        assert_eq!(r#"{}"#, format!("{}", m));
        m.insert("one".to_string());
        m.insert("two".to_string());
        assert_eq!(r#"{"one", "two"}"#, format!("{:?}", m));
        assert_eq!(r#"{one, two}"#, format!("{}", m));
    }
}
