// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use thiserror::Error as ThisError;

use crate::Row;

#[derive(Debug, ThisError, PartialEq)]
pub enum Error {
    #[error("key '{0}' not found")]
    KeyNotFound(String),
    #[error("key `{0}` does not match key of row `{1:?}`")]
    KeyValueMismatch(String, Row),
    #[error("key '{0}' already exists")]
    DuplicateKey(String),
    #[error("mutex poisoned: '{0}'")]
    MutexPoisoned(String),
    #[error("serde_json error occurred during serialization: '{0}'")]
    JsonSerialize(String),
    #[error("serde_json error occurred during deserialization: '{0}'")]
    JsonDeserialize(String),
}

impl Error {
    pub fn key_not_found(key: &str) -> Self {
        Error::KeyNotFound(key.to_string())
    }

    pub fn duplicate_key(key: &str) -> Self {
        Error::DuplicateKey(key.to_string())
    }

    pub fn mutex_poisoned<T>(err: &std::sync::PoisonError<T>) -> Self {
        Error::MutexPoisoned(err.to_string())
    }

    pub fn json_ser(err: &serde_json::Error) -> Self {
        Self::JsonSerialize(err.to_string())
    }

    pub fn json_de(err: &serde_json::Error) -> Self {
        Self::JsonDeserialize(err.to_string())
    }
}

impl<T> From<Error> for Result<T> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}

/// Simple result type used by all database operations.
pub type Result<T> = std::result::Result<T, Error>;
