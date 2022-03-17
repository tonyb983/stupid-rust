// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::OffsetDateTime;

mod dashmap_store;
mod disk;
mod hashmap_store;
mod row;

pub use dashmap_store::DashStore;
pub use disk::{RowDiskRepr, StoreByteRepr, StoreDiskRepr};
pub use hashmap_store::KeyValueStore;
pub use row::Row;

pub fn create_now() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

pub fn reverse_timestamp(input: i64) -> OffsetDateTime {
    match OffsetDateTime::from_unix_timestamp(input) {
        Ok(odt) => odt,
        Err(err) => panic!("error occurred reversing timestamp: {}", err),
    }
}

/// TODO: Generalize `KeyValueStore` to this trait, and allow for multiple
/// implementations of the `Store` to measure and compare performance.
/// First up will be implementing this as a `HashSet` instead of `HashMap` using
/// the newly added `Hash` implementation for `Row` (hashing based only on the key field).
pub trait Store {
    fn get_clone(&self, key: &str) -> crate::Result<Row>;
    fn insert(&self, key: &str, value: &str) -> crate::Result<()>;
    fn insert_row(&self, row: &Row) -> crate::Result<()>;
    fn set_or_insert(&self, key: &str, value: &str) -> crate::Result<()>;
    fn set_or_insert_row(&self, row: &Row) -> crate::Result<()>;
    fn contains(&self, key: &str) -> crate::Result<bool>;
    fn len(&self) -> crate::Result<usize>;
    fn delete(&self, key: &str) -> crate::Result<Row>;
    fn to_disk_repr(&self) -> crate::Result<StoreDiskRepr>;
    // fn from_disk_repr(disk_repr: &StoreDiskRepr) -> crate::Result<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;
}
