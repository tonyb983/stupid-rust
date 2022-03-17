// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod error;
mod mem_tbl;
mod wal;

pub use error::{Error, Result};
pub use mem_tbl::{KeyValueStore, Row, RowDiskRepr, StoreByteRepr, StoreDiskRepr};
