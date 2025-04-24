// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

use seq_macro::seq;
use std::hash::Hash;
use std::vec;

use micromap::Map;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;
use uuid::Builder;
use uuid::Uuid;

const ROUNDS: usize = 12345; // 123456 is ok, but a bit slower.

fn main() {
    each_capacity();
}

#[test]
fn each_capacity() {
    seq!(N in 0..257 {
        let micro_map: Map<Uuid, [u8; 16], N> = Map::new();
        let micromap_results = for_n::<N>(micro_map);
        // println!("[MicroMap] results for N={}: \n{:?}", N, micromap_results);
        let hash_map: HashMap<Uuid, [u8; 16]> = HashMap::new();
        let hashmap_results = for_n::<N>(hash_map);
        // println!("[HashMap ] results for N={}: \n{:?}", N, hashmap_results);
        assert_eq!(micromap_results, hashmap_results);
        // let result_hashes = results.iter().map(|result| make_hash(result, 0)).collect::<Vec<_>>();
        // hint::black_box(result_hashes);
        // println!("result_hashes for N={}: {:?}", N, result_hashes);
    });
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

// #[test]
// fn random_test() {
//     let mut rng = SmallRng::seed_from_u64(1);

//     let mut uuids = vec![];
//     for _ in 0..64 {
//         let random_bytes = rng.random();
//         let uuid = Builder::from_bytes(random_bytes).into_uuid();
//         uuids.push(uuid);
//     }

//     let key_index_list: Vec<usize> = (0..10000).map(|_| rng.random_range(0..64)).collect();
//     let value_list: Vec<[u8; 16]> = (0..10000).map(|_| rng.random()).collect();
//     let remove_key_index_list: Vec<usize> = (0..10000)
//         .step_by(20)
//         .map(|i| key_index_list[i + rng.random_range(0..20)])
//         .collect();
//     let remove_or_not_list: Vec<bool> = (0..10000).map(|_| rng.random_range(0..100) < 10).collect();
//     // let mut do_remove_after_insert_i: Vec<usize> = (0..remove_key_index_list.len()).map(|_| rng.random_range(0..10000)).collect();
//     // do_remove_after_insert_i.sort();
//     // println!("remove_key_index_list is {:?}", remove_key_index_list);

//     // get average of key_index_list
//     let mut sum = 0;
//     for i in 0..key_index_list.len() {
//         sum += key_index_list[i];
//     }
//     let avg = sum as f64 / key_index_list.len() as f64;
//     println!("key_index_list avg is {:?}", avg);

//     let mut sum = 0;

//     for _ in 0..1234 {
//         let mut m: Map<Uuid, [u8; 16], 64> = Map::new();

//         // let mut before_full = vec![];
//         // let mut not_full = true;
//         let mut rm_it = remove_key_index_list.iter();
//         for i in 0..10000 {
//             let index = key_index_list[i];
//             let key = uuids[index];
//             let value = value_list[i];

//             m.insert(black_box(key), black_box(value));

//             if remove_or_not_list[i] {
//                 let Some(rm_i) = rm_it.next() else {
//                     break;
//                 };
//                 let key = uuids[*rm_i];
//                 m.remove(black_box(&key));
//             }

//             // if not_full {
//             //     before_full.push(m.len());
//             //     if m.len() == m.capacity() {
//             //         not_full = false;
//             //         println!(
//             //             "before_full(len={}) is {:?}",
//             //             before_full.len(),
//             //             before_full
//             //         );
//             //     }
//             // }
//         }

//         sum += m.len();
//     }
//     println!("sum is {}", sum);
// }
