// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Map;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::marker::PhantomData;

impl<K: PartialEq + Serialize, V: Serialize, const N: usize> Serialize for Map<K, V, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (a, v) in self.iter() {
            map.serialize_entry(a, v)?;
        }
        map.end()
    }
}

struct Vi<K, V, const N: usize>(PhantomData<K>, PhantomData<V>);

impl<'de, K: PartialEq + Deserialize<'de>, V: Deserialize<'de>, const N: usize> Visitor<'de>
    for Vi<K, V, N>
{
    type Value = Map<K, V, N>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut m: Self::Value = Map::new();
        while let Some((key, value)) = access.next_entry()? {
            m.insert(key, value);
        }
        Ok(m)
    }
}

impl<'de, K: PartialEq + Deserialize<'de>, V: Deserialize<'de>, const N: usize> Deserialize<'de>
    for Map<K, V, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi(PhantomData, PhantomData))
    }
}

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn serialize_and_deserialize() {
    let mut before: Map<u8, u8, 8> = Map::new();
    before.insert(1, 42);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8, u8, 8> = deserialize(&bytes).unwrap();
    assert_eq!(42, after.into_iter().next().unwrap().1);
}

#[test]
fn empty_map_serde() {
    let before: Map<u8, u8, 8> = Map::new();
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8, u8, 8> = deserialize(&bytes).unwrap();
    assert!(after.is_empty());
}
