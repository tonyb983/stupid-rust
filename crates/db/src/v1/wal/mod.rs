// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// TODO: Research and implement WAL
pub struct Wal {
    base_seq: i64,
    seq: i64,
    dir_path: String,
    file: std::fs::File,
}
