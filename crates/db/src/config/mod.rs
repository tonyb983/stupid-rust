// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use config::{Config, ConfigError, Environment as ConfigEnv, File as ConfigFile};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WalConfig {
    use_wal: bool,
}

impl Default for WalConfig {
    fn default() -> Self {
        Self { use_wal: false }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataConfig {
    save_to_disk: bool,
    save_path: Option<String>,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            save_to_disk: false,
            save_path: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    debug: bool,
    data: DataConfig,
    wal: WalConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("SDB_RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .set_default("debug", run_mode == "development")?
            .add_source(ConfigFile::with_name("config/default"))
            .add_source(ConfigFile::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(ConfigEnv::with_prefix("SDB").ignore_empty(true));
        if let Some(project_dir) = ProjectDirs::from("io", "imtony", "sdb") {
            builder = builder
                .set_default(
                    "data.save_path",
                    format!("{}", project_dir.data_dir().display()).as_str(),
                )?
                .add_source(ConfigFile::with_name(
                    format!("{}", project_dir.config_dir().join("config").display()).as_str(),
                ));
        }

        let settings = builder.build()?;

        settings.try_deserialize()
    }
}
