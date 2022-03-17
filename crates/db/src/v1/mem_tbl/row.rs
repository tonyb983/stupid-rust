// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) created: i64,
    pub(crate) updated: i64,
}

impl Row {
    /// Gets a reference to the `key` of this `Row`.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Gets a reference to the `value` of this `Row`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Gets the `created` timestamp value of this `Row`.
    pub fn created(&self) -> i64 {
        self.created
    }

    /// Gets the `updated` timestamp value of this `Row`.
    pub fn updated(&self) -> i64 {
        self.updated
    }

    /// Creates a new `Row` object with the given values. Use this to create a
    /// row object that matches data you already have on hand. Use `Row::create`
    /// when a new `Row` is being created by the user.
    pub fn new(key: &str, value: &str, created: i64, updated: i64) -> Self {
        Row {
            key: key.to_string(),
            value: value.to_string(),
            created,
            updated,
        }
    }

    /// Creates a new Row with the given `key` and `value`, setting `created`
    /// and `updated` to the current time. Use `Row::new` to create a row with
    /// full control over the `created` and `updated` fields.
    pub fn create<Key: AsRef<str>, Value: AsRef<str>>(key: Key, value: Value) -> Self {
        let now = super::create_now();
        Self {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
            created: now,
            updated: now,
        }
    }

    /// Updates the `value` of this `Row` and sets `updated` to the current
    /// timestamp.
    pub fn update<Value: AsRef<str>>(&mut self, value: Value) {
        let value = value.as_ref();
        if value != &self.value {
            self.value = value.to_string();
            self.updated = super::create_now();
        }
    }

    /// Clears the `value` of this row and changes `updated` to the current timestamp.
    pub fn clear(&mut self) {
        self.value = "".to_string();
        self.updated = super::create_now();
    }

    /// Updates the `updated` field to the current timestamp without changing
    /// any other data.
    pub fn touch(&mut self) {
        self.updated = super::create_now();
    }

    /// Overwrites all values in this [`Row`] with the values from `other`.
    pub fn overwrite_with(&mut self, other: &Row) {
        self.key = other.key.clone();
        self.value = other.value.clone();
        self.created = other.created;
        self.updated = other.updated;
    }
}

impl std::hash::Hash for Row {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

impl From<crate::rpc::RowData> for Row {
    fn from(data: crate::rpc::RowData) -> Self {
        Self {
            key: data.key,
            value: data.value,
            created: data.created,
            updated: data.updated,
        }
    }
}

impl From<Row> for crate::rpc::RowData {
    fn from(row: Row) -> Self {
        Self {
            key: row.key,
            value: row.value,
            created: row.created,
            updated: row.updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::create_now;
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use utils::assert_within;

    #[test]
    fn new() {
        let now = create_now();
        let row = Row::new("key", "value", now, now);
        assert_str_eq!(row.key(), "key");
        assert_str_eq!(row.value(), "value");
        assert_within!(row.created(), now - 1, now + 1);
        assert_within!(row.updated(), now - 1, now + 1);
    }

    #[test]
    fn create() {
        let now = create_now();
        let row = Row::create("key", "value");
        assert_str_eq!(row.key(), "key");
        assert_str_eq!(row.value(), "value");
        assert_within!(row.created(), now - 1, now + 1);
        assert_within!(row.updated(), now - 1, now + 1);
    }
}
