// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, sync::Mutex};

use super::row::Row;

pub type Data = HashMap<String, Row>;

#[derive(Debug, Default)]
pub struct KeyValueStore {
    data: Mutex<Data>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_clone(&self, key: &str) -> Option<Row> {
        self.data
            .lock()
            .expect("Mutex is poisoned")
            .get(key)
            .cloned()
    }

    pub fn insert(&self, key: &str, value: &str) -> Option<()> {
        let mut lock = self.data.lock().expect("Mutex is poisoned");

        let k = key.to_string();

        if lock.contains_key(&k) {
            return None;
        }

        lock.insert(k.to_string(), Row::create(key, value));
        Some(())
    }

    pub fn set_or_insert(&self, key: &str, value: &str) {
        let mut lock = self.data.lock().expect("Mutex is poisoned");

        let k = key.to_string();

        if lock.contains_key(&k) {
            lock.entry(k)
                .and_modify(|v| v.update(value))
                .or_insert(Row::create(key, value));
        } else {
            lock.insert(k, Row::create(key, value));
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data
            .lock()
            .expect("Mutex is poisoned")
            .contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = KeyValueStore::new();
        assert!(!store.contains("key"));
        let result = store.insert("key", "value");
        assert!(result.is_some());
        assert!(store.contains("key"));
        let result = store.insert("key", "whoops");
        assert!(result.is_none());
        store.set_or_insert("key", "whoops");
        let row = store.get_clone("key").expect("Unable to get row");
        assert_eq!(row.value(), "whoops");
    }
}
