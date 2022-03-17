// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, sync::Mutex};

use crate::{Row, RowDiskRepr, StoreByteRepr, StoreDiskRepr};

pub type Data = HashMap<String, Row>;

#[derive(Debug, Default)]
pub struct KeyValueStore {
    data: Mutex<Data>,
}

impl KeyValueStore {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get_clone(&self, key: &str) -> crate::Result<Row> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|data| {
                data.get(key)
                    .cloned()
                    .ok_or(crate::Error::key_not_found(key))
            })
    }

    pub fn insert(&self, key: &str, value: &str) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|mut data| {
                if data.contains_key(key) {
                    Err(crate::Error::duplicate_key(key))
                } else {
                    data.insert(key.to_string(), Row::create(key, value));
                    Ok(())
                }
            })
    }

    pub fn insert_row(&self, row: &Row) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|mut data| {
                let key = row.key().to_string();
                if data.contains_key(&key) {
                    Err(crate::Error::duplicate_key(row.key()))
                } else {
                    data.insert(key, row.clone());
                    Ok(())
                }
            })
    }

    pub fn set_or_insert(&self, key: &str, value: &str) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|mut data| {
                let k = key.to_string();
                data.entry(k)
                    .and_modify(|v| v.update(value))
                    .or_insert(Row::create(key, value));
                Ok(())
            })
    }

    pub fn set_or_insert_row(&self, row: &Row) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|mut data| {
                // TODO: Is this 'to_string' avoidable?
                data.entry(row.key().to_string())
                    .and_modify(|v| v.overwrite_with(row))
                    .or_insert(row.clone());
                Ok(())
            })
    }

    pub fn contains(&self, key: &str) -> crate::Result<bool> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .map(|data| data.contains_key(key))
    }

    pub fn len(&self) -> crate::Result<usize> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .map(|data| data.len())
    }

    pub fn delete(&self, key: &str) -> crate::Result<Row> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|mut data| data.remove(key).ok_or(crate::Error::key_not_found(key)))
    }

    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .and_then(|data| serde_json::to_vec(&*data).map_err(|err| crate::Error::json_ser(&err)))
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        serde_json::from_slice(bytes)
            .map_err(|err| crate::Error::json_de(&err))
            .map(|data| Self {
                data: Mutex::new(data),
            })
    }

    pub fn to_disk(&self) -> crate::Result<StoreDiskRepr> {
        self.data
            .lock()
            .map_err(|err| crate::Error::mutex_poisoned(&err))
            .map(|data| data.values().cloned().collect::<Vec<_>>())
            .map(|rows| rows.into())
    }

    pub fn into_disk(self) -> crate::Result<StoreDiskRepr> {
        let disk = self
            .data
            .into_inner()
            .unwrap_or_else(|e| e.into_inner())
            .into_values()
            .collect::<Vec<_>>()
            .into();
        Ok(disk)
    }

    pub fn from_disk(disk: &StoreDiskRepr) -> crate::Result<Self> {
        todo!()
    }
}

impl super::Store for KeyValueStore {
    fn get_clone(&self, key: &str) -> crate::Result<Row> {
        KeyValueStore::get_clone(self, key)
    }

    fn insert(&self, key: &str, value: &str) -> crate::Result<()> {
        KeyValueStore::insert(self, key, value)
    }

    fn insert_row(&self, row: &Row) -> crate::Result<()> {
        KeyValueStore::insert_row(self, row)
    }

    fn set_or_insert(&self, key: &str, value: &str) -> crate::Result<()> {
        KeyValueStore::set_or_insert(self, key, value)
    }

    fn set_or_insert_row(&self, row: &Row) -> crate::Result<()> {
        KeyValueStore::set_or_insert_row(self, row)
    }

    fn contains(&self, key: &str) -> crate::Result<bool> {
        KeyValueStore::contains(self, key)
    }

    fn len(&self) -> crate::Result<usize> {
        KeyValueStore::len(self)
    }

    fn delete(&self, key: &str) -> crate::Result<Row> {
        KeyValueStore::delete(self, key)
    }

    fn to_disk_repr(&self) -> crate::Result<StoreDiskRepr> {
        KeyValueStore::to_disk_repr(self)
    }
}

impl<'s> FromIterator<(&'s str, Row)> for KeyValueStore {
    fn from_iter<T: IntoIterator<Item = (&'s str, Row)>>(iter: T) -> Self {
        let mut data: HashMap<String, Row> =
            iter.into_iter().map(|(s, r)| (s.to_string(), r)).collect();
        Self {
            data: Mutex::new(data),
        }
    }
}

impl<'t, 's: 't> FromIterator<&'t (&'s str, Row)> for KeyValueStore {
    fn from_iter<T: IntoIterator<Item = &'t (&'s str, Row)>>(iter: T) -> Self {
        let mut data: HashMap<String, Row> = iter
            .into_iter()
            .map(|(s, r)| (s.to_string(), r.clone()))
            .collect();
        Self {
            data: Mutex::new(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    mod helpers {
        use super::super::*;
        use pretty_assertions::{assert_eq, assert_ne};

        pub fn store_with(values: &[(&str, &str)]) -> KeyValueStore {
            let values = values.to_vec();
            let mut store = KeyValueStore::empty();
            for &(key, value) in &values {
                assert!(
                    store.insert(key, value).is_ok(),
                    "store_with - failed to insert ({}, {})",
                    key,
                    value
                );
            }

            assert_eq!(
                store.len().expect("store_with - failed to get length"),
                values.len()
            );

            store
        }

        pub fn fill_single_thread(values: usize) -> KeyValueStore {
            let store = KeyValueStore::empty();
            for i in 0..values {
                let key = format!("key{}", i);
                let value = format!("value{}", i);
                assert!(
                    store.insert(key.as_str(), value.as_str()).is_ok(),
                    "fill_single_thread - unable to insert ({},{})",
                    key,
                    value
                );
            }

            assert_eq!(
                store
                    .len()
                    .expect("fill_single_thread - unable to get length"),
                values,
                "fill_single_thread - did not add the expected number of values",
            );

            store
        }

        pub fn fill_multi_thread(values: usize, threads: usize) -> KeyValueStore {
            use std::sync::Arc;
            use std::thread;

            if values == 0 {
                eprintln!("fill_multi_thread - called with values = 0");
                return KeyValueStore::empty();
            }

            if threads < 2 {
                return fill_single_thread(values);
            }

            let step_size = values / threads;
            let store = Arc::new(KeyValueStore::empty());
            let mut ts = Vec::new();

            for t in 0..(threads - 1) {
                let clone = Arc::clone(&store);
                let start = t * step_size;
                let end = start + step_size;
                // println!("Starting thread #{} with range {}..{}", t + 1, start, end);
                ts.push(thread::spawn(move || {
                    for i in start..end {
                        let key = format!("key{}", i);
                        let value = format!("value{}", i);
                        assert!(
                            clone.insert(key.as_str(), value.as_str()).is_ok(),
                            "fill_multi_thread - T{} - unable to insert ({},{})",
                            t + 1,
                            key,
                            value
                        );
                    }
                    drop(clone);
                }));
            }

            for i in ((threads - 1) * step_size)..values {
                let key = format!("key{}", i);
                let value = format!("value{}", i);
                assert!(
                    store.insert(key.as_str(), value.as_str()).is_ok(),
                    "fill_multi_thread - unable to insert ({},{})",
                    key,
                    value
                );
            }

            let mut i = 1;
            for handle in ts.into_iter() {
                handle.join().expect(
                    format!("fill_multi_thread - unable to join thread {}", i + 1).as_str(),
                );
                i += 1;
            }

            let inner: KeyValueStore =
                Arc::try_unwrap(store).expect("fill_multi_thread - unable to take inner store");

            assert_eq!(
                inner
                    .len()
                    .expect("fill_multi_thread - unable to get length"),
                values,
                "fill_multi_thread - did not add the expected number of values",
            );
            inner
        }
    }

    #[test]
    fn collect() {
        let data = vec![
            ("key1", Row::create("key1", "value1")),
            ("key2", Row::create("key2", "value2")),
            ("key3", Row::create("key3", "value3")),
        ];

        let store: KeyValueStore = data.iter().collect();
    }

    #[test]
    fn it_works() {
        let store = KeyValueStore::empty();
        let result = store.contains("key");
        assert!(result.is_ok());
        assert!(!result.unwrap());
        let result = store.insert("key", "value");
        assert!(result.is_ok());
        let result = store.contains("key");
        assert!(result.is_ok());
        assert!(result.unwrap());
        let result = store.insert("key", "whoops");
        assert!(result.is_err());
        let result = store.set_or_insert("key", "whoops");
        assert!(result.is_ok());
        let result = store.get_clone("key");
        assert!(result.is_ok());
        let row = result.unwrap();
        assert_eq!(row.value(), "whoops");
        let result = store.len();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        let result = store.delete("key");
        assert!(result.is_ok());
        let result = store.delete("key");
        assert!(result.is_err());
    }

    #[test]
    fn byte_roundtrip() {
        let original = KeyValueStore::empty();
        assert!(original.insert("key1", "value1").is_ok());
        assert!(original.insert("key2", "value2").is_ok());
        assert!(original.insert("key3", "value3").is_ok());
        assert_eq!(
            original
                .len()
                .expect("byte_roundtrip (original) - unable to get length"),
            3
        );
        assert_eq!(
            original
                .get_clone("key1")
                .expect("byte_roundtrip (original) - unable to get key1")
                .value(),
            "value1"
        );
        assert_eq!(
            original
                .get_clone("key2")
                .expect("byte_roundtrip (original) - unable to get key2")
                .value(),
            "value2"
        );
        assert_eq!(
            original
                .get_clone("key3")
                .expect("byte_roundtrip (original) - unable to get key3")
                .value(),
            "value3"
        );
        let result = original.to_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(original.insert("key4", "value4").is_ok());
        assert_eq!(
            original
                .len()
                .expect("byte_roundtrip (original) - unable to get length"),
            4
        );
        let result = KeyValueStore::from_bytes(&bytes);
        assert!(result.is_ok());
        let clone = result.unwrap();
        assert_eq!(
            clone
                .len()
                .expect("byte_roundtrip (clone) - unable to get length"),
            3
        );
        assert_eq!(
            clone
                .get_clone("key1")
                .expect("byte_roundtrip (clone) - unable to get key1")
                .value(),
            "value1"
        );
        assert_eq!(
            clone
                .get_clone("key2")
                .expect("byte_roundtrip (clone) - unable to get key2")
                .value(),
            "value2"
        );
        assert_eq!(
            clone
                .get_clone("key3")
                .expect("byte_roundtrip (clone) - unable to get key3")
                .value(),
            "value3"
        );
        assert!(clone.get_clone("key4").is_err());
    }

    #[test]
    fn tempfile_roundtrip() {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom, Write};

        let original = KeyValueStore::empty();
        // Load and assert original
        {
            assert!(original.insert("key1", "value1").is_ok());
            assert!(original.insert("key2", "value2").is_ok());
            assert!(original.insert("key3", "value3").is_ok());
            assert_eq!(
                original
                    .len()
                    .expect("tempfile_roundtrip (original) - unable to get length"),
                3
            );
            assert_eq!(
                original
                    .get_clone("key1")
                    .expect("tempfile_roundtrip (original) - unable to get key1")
                    .value(),
                "value1"
            );
            assert_eq!(
                original
                    .get_clone("key2")
                    .expect("tempfile_roundtrip (original) - unable to get key2")
                    .value(),
                "value2"
            );
            assert_eq!(
                original
                    .get_clone("key3")
                    .expect("tempfile_roundtrip (original) - unable to get key3")
                    .value(),
                "value3"
            );
        }

        let result = original.to_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();
        let mut tempfile = tempfile::tempfile().expect("Unable to open tempfile");
        tempfile
            .write_all(&bytes)
            .expect("Unable to write to tempfile");

        assert!(original.insert("key4", "value4").is_ok());
        assert_eq!(
            original
                .len()
                .expect("tempfile_roundtrip (original) - unable to get length"),
            4
        );

        tempfile
            .seek(SeekFrom::Start(0))
            .expect("Unable to seek to start in tempfile");
        let rbytes = tempfile
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .expect("Unable to read tempfile");

        let result = KeyValueStore::from_bytes(&rbytes);
        assert!(result.is_ok());
        let clone = result.unwrap();

        // Assert clone
        {
            assert_eq!(
                clone
                    .len()
                    .expect("tempfile_roundtrip (clone) - unable to get length"),
                3
            );
            assert_eq!(
                clone
                    .get_clone("key1")
                    .expect("tempfile_roundtrip (clone) - unable to get key1")
                    .value(),
                "value1"
            );
            assert_eq!(
                clone
                    .get_clone("key2")
                    .expect("tempfile_roundtrip (clone) - unable to get key2")
                    .value(),
                "value2"
            );
            assert_eq!(
                clone
                    .get_clone("key3")
                    .expect("tempfile_roundtrip (clone) - unable to get key3")
                    .value(),
                "value3"
            );
            assert!(clone.get_clone("key4").is_err());
        }
    }

    #[test]
    fn check_fill_single() {
        use helpers::fill_single_thread;
        let mut vals: usize = 100;
        let store = fill_single_thread(vals);
        assert_eq!(
            store
                .len()
                .expect("check_fill_single - unable to get length"),
            vals,
            "check_fill_single - did not add the expected number of values",
        );

        vals = 200;
        let store = fill_single_thread(vals);
        assert_eq!(
            store
                .len()
                .expect("check_fill_single - unable to get length"),
            vals,
            "check_fill_single - did not add the expected number of values",
        );
    }

    #[test]
    fn check_fill_multi() {
        use helpers::fill_multi_thread;
        let mut vals: usize = 100;
        let mut threads: usize = 1;

        let (vals, threads) = (100, 1);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (100, 2);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (100, 3);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (100, 4);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (177, 3);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 1);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 2);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 3);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 4);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 5);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 6);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 7);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 8);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 9);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (10000, 10);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (50, 12);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );

        let (vals, threads) = (20, 20);
        let store = fill_multi_thread(vals, threads);
        assert_eq!(
            store
                .len()
                .expect("check_fill_multi_thread - unable to get length"),
            vals,
            "check_fill_multi_thread (v = {} t = {}) - did not add the expected number of values",
            vals,
            threads
        );
    }

    #[test]
    fn check_fill_multi_rng() {
        use helpers::fill_multi_thread;
        use std::iter::repeat_with;
        const ITERS: usize = 25;

        let mut i: usize = 0;
        let mut next = move || {
            i += 1;
            i
        };

        let rand_v = || fastrand::usize(100..40000);
        let rand_t = || fastrand::usize(1..10);

        repeat_with(|| (next(), rand_v(), rand_t()))
            .take(ITERS)
            .for_each(|(n, vals, threads)| {
                println!("Iteration #{} with {} values and {} threads", n, vals, threads);
                let store = fill_multi_thread(vals, threads);
                assert_eq!(
                    store
                        .len()
                        .expect("check_fill_multi_random - unable to get length"),
                    vals,
                    "check_fill_multi_random (v = {} t = {}) - did not add the expected number of values",
                    vals,
                    threads
                );
                let r = fastrand::usize(0..vals);
                let key = format!("key{}", r);
                let value = format!("value{}", r);
                let row = store.get_clone(&key);
                assert!(row.is_ok(), "check_fill_multi_random ({},{},{}) - unable to get value", n, vals, threads);
                let row = row.unwrap();
                assert_eq!(
                    row.key(),
                    key,
                    "check_fill_multi_random #{} - row {} does not have the expected key",
                    n,
                    r,
                );
                assert_eq!(
                    row.value(),
                    value,
                    "check_fill_multi_random #{} - row {} does not have the expected value",
                    n,
                    r,
                );
            });
    }
}
