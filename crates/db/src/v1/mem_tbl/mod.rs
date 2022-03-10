// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::OffsetDateTime;

mod row;
mod store;

pub fn create_now() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

pub fn reverse_timestamp(input: i64) -> OffsetDateTime {
    match OffsetDateTime::from_unix_timestamp(input) {
        Ok(odt) => odt,
        Err(err) => panic!("error occurred reversing timestamp: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
