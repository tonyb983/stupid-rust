// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row {
    key: String,
    value: String,
    created: i64,
    updated: i64,
}

impl Row {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn created(&self) -> i64 {
        self.created
    }

    pub fn updated(&self) -> i64 {
        self.updated
    }

    pub fn new(key: &str, value: &str, created: i64, updated: i64) -> Self {
        Row {
            key: key.to_string(),
            value: value.to_string(),
            created,
            updated,
        }
    }

    pub fn create<Key: AsRef<str>, Value: AsRef<str>>(key: Key, value: Value) -> Self {
        let now = super::create_now();
        Self {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
            created: now,
            updated: now,
        }
    }

    pub fn update<Value: AsRef<str>>(&mut self, value: Value) {
        let value = value.as_ref();
        if value != &self.value {
            self.value = value.to_string();
            self.updated = super::create_now();
        }
    }

    pub fn clear(&mut self) {
        self.value = "".to_string();
        self.updated = super::create_now();
    }

    pub fn touch(&mut self) {
        self.updated = super::create_now();
    }
}
