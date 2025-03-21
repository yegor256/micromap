// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Set;
use core::fmt::{self, Debug, Formatter};

impl<T: PartialEq + Debug, const N: usize> Debug for Set<T, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn debugs_set() {
        let mut m: Set<String, 10> = Set::new();
        m.insert("one".to_string());
        m.insert("two".to_string());
        assert_eq!(r#"{"one", "two"}"#, format!("{:?}", m));
    }

    #[test]
    fn debug_alternate_set() {
        let mut m: Set<String, 10> = Set::new();
        m.insert("one".to_string());
        m.insert("two".to_string());
        assert_eq!(
            r#"{
    "one",
    "two",
}"#,
            format!("{:#?}", m)
        );
    }
}
