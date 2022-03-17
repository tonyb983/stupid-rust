// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Row;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RowDiskRepr {
    pub key: String,
    pub value: String,
    pub created: i64,
    pub updated: i64,
}

impl From<Row> for RowDiskRepr {
    fn from(row: Row) -> Self {
        Self {
            key: row.key().to_string(),
            value: row.value().to_string(),
            created: row.created(),
            updated: row.updated(),
        }
    }
}

impl From<&Row> for RowDiskRepr {
    fn from(row: &Row) -> Self {
        Self {
            key: row.key().to_string(),
            value: row.value().to_string(),
            created: row.created(),
            updated: row.updated(),
        }
    }
}

impl From<RowDiskRepr> for Row {
    fn from(row: RowDiskRepr) -> Self {
        Self {
            key: row.key,
            value: row.value,
            created: row.created,
            updated: row.updated,
        }
    }
}

impl From<&RowDiskRepr> for Row {
    fn from(row: &RowDiskRepr) -> Self {
        let RowDiskRepr {
            key,
            value,
            created,
            updated,
        } = row.clone();
        Self {
            key,
            value,
            created,
            updated,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoreDiskRepr {
    pub version: u8,
    pub data: Vec<RowDiskRepr>,
}

impl StoreDiskRepr {
    const VERSION: u8 = 1;
    pub const fn current_version() -> u8 {
        Self::VERSION
    }

    pub fn new(data: &[RowDiskRepr]) -> Self {
        Self::from_vec(data.to_vec())
    }

    pub fn from_vec(data: Vec<RowDiskRepr>) -> Self {
        Self {
            version: Self::current_version(),
            data,
        }
    }
}

impl<'row> FromIterator<&'row Row> for StoreDiskRepr {
    fn from_iter<T: IntoIterator<Item = &'row Row>>(iter: T) -> Self {
        let rows: Vec<RowDiskRepr> = iter.into_iter().map(|r| r.into()).collect();
        Self::from_vec(rows)
    }
}

impl FromIterator<Row> for StoreDiskRepr {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self {
        let rows: Vec<RowDiskRepr> = iter.into_iter().map(|r| r.into()).collect();
        Self::from_vec(rows)
    }
}

impl<'row> FromIterator<&'row RowDiskRepr> for StoreDiskRepr {
    fn from_iter<T: IntoIterator<Item = &'row RowDiskRepr>>(iter: T) -> Self {
        let rows: Vec<RowDiskRepr> = iter.into_iter().map(|r| r.clone()).collect();
        Self::from_vec(rows)
    }
}

impl FromIterator<RowDiskRepr> for StoreDiskRepr {
    fn from_iter<T: IntoIterator<Item = RowDiskRepr>>(iter: T) -> Self {
        let rows: Vec<RowDiskRepr> = iter.into_iter().collect();
        Self::from_vec(rows)
    }
}

impl From<Vec<Row>> for StoreDiskRepr {
    fn from(rows: Vec<Row>) -> Self {
        Self::from_iter(rows.into_iter())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoreByteRepr {
    pub version: u8,
    pub data: Vec<u8>,
}

impl StoreByteRepr {
    const VERSION: u8 = 1;
    pub const fn current_version() -> u8 {
        Self::VERSION
    }

    pub fn new(data: &[u8]) -> Self {
        Self {
            version: Self::VERSION,
            data: data.to_vec(),
        }
    }
}
