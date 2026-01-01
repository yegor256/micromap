// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

#![cfg(debug_assertions)]

use seq_macro::seq;
use std::hash::Hash;
use std::vec;

use indexmap::IndexMap;
use micromap::Map;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;
use uuid::Builder;
use uuid::Uuid;

// This is a deterministic random test.
// DON'T change the code unless you know what you are doing.

const ROUNDS: usize = 12345; // 123456 is ok, but a bit slower. (XOR_EXPECT=0xFD9A2154FC6C60DD)
const XOR_EXPECT: u64 = 0x150D46E005A17B7C; // for a specific ROUNDS=12345

/// Run this by:
/// `$ cargo test --test random (without --release)`
///
/// Note: only run in debug mode (opt-level=0), cause this is a deterministic result, it will be
/// calculated at compile time when optimization is enabled and it will stuck.
#[test]
fn each_capacity() {
    let mut xor_hash = 0u64;
    seq!(N in 0..257 {
        let micro_map: Map<Uuid, [u8; 16], N> = Map::new();
        let micromap_results = for_n::<N>(micro_map);
        let hash_map: HashMap<Uuid, [u8; 16]> = HashMap::new();
        let hashmap_results = for_n::<N>(hash_map);
        assert_eq!(micromap_results, hashmap_results);
        // Or use IndexMap instead of HashMap if you want, the final
        // result sequence is consistent.
        // let index_map: IndexMap<Uuid, [u8; 16]> = IndexMap::new();
        // let indexmap_results = for_n::<N>(index_map);
        // assert_eq!(indexmap_results, micromap_results);
        for result in micromap_results {
            let hash_val = make_hash(&result, ROUNDS as u64);
            xor_hash ^= hash_val;
        }
    });
    assert_eq!(
        xor_hash, XOR_EXPECT,
        "xor_hash=0x{:0X} is not eq to XOR_EXPECT=0x{:0X} for ROUNDS={}",
        xor_hash, XOR_EXPECT, ROUNDS
    );
}

trait IsMap<K, V, const N: usize>
where
    Self: Default + Sized,
{
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool;
    fn len(&self) -> usize;
    fn contains_key(&self, key: &K) -> bool;
    fn clear(&mut self);
}

impl<K, V, const N: usize> IsMap<K, V, N> for Map<K, V, N>
where
    K: PartialEq,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.retain(f)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K, V, const N: usize> IsMap<K, V, N> for HashMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.retain(f)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<K, V, const N: usize> IsMap<K, V, N> for IndexMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.swap_remove(key)
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.retain(f)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

enum MapOp<K, V> {
    Get(K),
    Insert(K, V),
    Remove(K),
    Retain(Box<dyn Fn(&K, &mut V) -> bool>),
    GetLen,
    ContainsKey(K),
    Clear,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum OpResult<K, V> {
    Len(usize),
    Key(Option<K>),
    Value(Option<V>),
    KeyValue(Option<(K, V)>),
    Boolean(bool),
    None,
}

impl MapOp<Uuid, [u8; 16]> {
    fn apply<'a, 'b, const N: usize>(
        self,
        map: &mut impl IsMap<Uuid, [u8; 16], N>,
    ) -> OpResult<Uuid, [u8; 16]> {
        match self {
            MapOp::Get(key) => {
                let value = map.get(&key);
                OpResult::Value(value.cloned())
            }
            MapOp::Insert(key, value) => {
                let old_value = map.insert(key, value);
                OpResult::Value(old_value)
            }
            MapOp::Remove(key) => {
                let taken_value = map.remove(&key);
                OpResult::Value(taken_value)
            }
            MapOp::Retain(f) => {
                map.retain(f);
                OpResult::None
            }
            MapOp::GetLen => {
                let len = map.len();
                OpResult::Len(len)
            }
            MapOp::ContainsKey(key) => {
                let yes = map.contains_key(&key);
                OpResult::Boolean(yes)
            }
            MapOp::Clear => {
                map.clear();
                OpResult::None
            }
        }
    }
}

fn for_n<const N: usize>(mut map: impl IsMap<Uuid, [u8; 16], N>) -> Vec<OpResult<Uuid, [u8; 16]>> {
    // create a random number generator with a seed based on N
    let mut rng = SmallRng::seed_from_u64(N as u64);
    // create a pool with a specific number of unique UUIDs
    let mut uuids = vec![];
    let uuids_len = usize::max(64, N);
    for _ in 0..uuids_len {
        let random_bytes = rng.random();
        let uuid = Builder::from_bytes(random_bytes).into_uuid();
        uuids.push(uuid);
    }
    // create the random operation sequence for the map
    let mut map_ops: Vec<MapOp<Uuid, [u8; 16]>> = Vec::new();
    // for empty map
    map_ops.push(MapOp::GetLen);
    map_ops.push(MapOp::Clear);
    map_ops.push(MapOp::ContainsKey(uuids[rng.random_range(0..uuids_len)]));
    map_ops.push(MapOp::Remove(uuids[rng.random_range(0..uuids_len)]));
    map_ops.push(MapOp::Get(uuids[rng.random_range(0..uuids_len)]));
    map_ops.push(MapOp::GetLen);
    map_ops.push(MapOp::Retain(Box::new(|k, _| make_hash(k, 0) % 2 == 0)));
    // for one element map
    if N > 0 {
        map_ops.push(MapOp::Insert(uuids[0], rng.random()));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Insert(uuids[0], rng.random()));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Get(uuids[rng.random_range(0..uuids_len)]));
        map_ops.push(MapOp::Retain(Box::new(|k, _| make_hash(k, 0) % 2 == 0)));
        map_ops.push(MapOp::ContainsKey(uuids[1]));
        map_ops.push(MapOp::Remove(uuids[1]));
        map_ops.push(MapOp::ContainsKey(uuids[1]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Remove(uuids[1]));
        map_ops.push(MapOp::Remove(uuids[0]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Remove(uuids[0]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Clear);
    }
    // for three elements map
    if N >= 3 {
        map_ops.push(MapOp::Insert(uuids[0], rng.random()));
        map_ops.push(MapOp::Insert(uuids[1], rng.random()));
        map_ops.push(MapOp::Insert(uuids[2], rng.random()));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::ContainsKey(uuids[2]));
        map_ops.push(MapOp::Get(uuids[3]));
        map_ops.push(MapOp::Get(uuids[2]));
        map_ops.push(MapOp::Get(uuids[0]));
        map_ops.push(MapOp::Get(uuids[1]));
        map_ops.push(MapOp::Remove(uuids[0]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Remove(uuids[2]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::ContainsKey(uuids[2]));
        map_ops.push(MapOp::Remove(uuids[0]));
        map_ops.push(MapOp::Remove(uuids[2]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Remove(uuids[1]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Retain(Box::new(|k, _| make_hash(k, 0) % 2 == 0)));
        map_ops.push(MapOp::Clear);
    }
    // for half-full map (N >= 4)
    if N >= 4 {
        for uuid in uuids.iter().take(N / 2) {
            map_ops.push(MapOp::Insert(*uuid, uuid.into_bytes()));
        }
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Remove(uuids[rng.random_range(0..uuids_len)]));
        map_ops.push(MapOp::Remove(uuids[1]));
        map_ops.push(MapOp::Retain(Box::new(|k, _| make_hash(k, 0) % 2 == 0)));
        map_ops.push(MapOp::ContainsKey(uuids[rng.random_range(0..uuids_len)]));
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Clear);
    }
    // for almost-full map (N >= 3)
    if N >= 3 {
        for uuid in uuids.iter().take(N - 1) {
            map_ops.push(MapOp::Insert(*uuid, uuid.into_bytes()));
        }
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Retain(Box::new(|k, _| make_hash(k, 0) % 2 == 0)));
        map_ops.push(MapOp::GetLen);
        for uuid in uuids.iter() {
            map_ops.push(MapOp::Remove(*uuid));
        }
        map_ops.push(MapOp::GetLen);
        map_ops.push(MapOp::Clear);
    }
    // for full map (N >= 3)
    if N >= 3 {
        for _ in 0..ROUNDS {
            let i = rng.random_range(0..N);
            let uuid = uuids[i].clone();
            if rng.random_ratio(85, 100) {
                map_ops.push(MapOp::Insert(uuid, uuid.into_bytes()));
            } else {
                map_ops.push(MapOp::Remove(uuid));
            }
            if rng.random_ratio(10, 100) {
                map_ops.push(MapOp::GetLen);
            }
        }
        map_ops.push(MapOp::Clear);
    }
    // apply the operation sequence to the map
    let mut results = Vec::new();
    for op in map_ops {
        results.push(op.apply::<N>(&mut map));
    }
    return results;
}

fn make_hash<K: core::hash::Hash>(k: &K, seed: u64) -> u64 {
    use core::hash::BuildHasher;
    let state = foldhash::fast::FixedState::with_seed(seed);
    state.hash_one(k)
}
