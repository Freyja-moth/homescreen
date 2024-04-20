use std::fs;

use actix_web::web::Data;
use homescreen_errors::prelude::*;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct Config {
    port: u16,
    database_url: String,
}
impl Config {
    pub fn load() -> HomescreenResult<Self> {
        debug!("Loading config");

        fs::read_to_string("Config.toml")
            .map_err(ConfigError::CannotFindConfigFile)
            .inspect(|_| trace!("Loaded config"))
            .and_then(|config| {
                toml::from_str(&config)
                    .map_err(ConfigError::CannotParseConfigFile)
                    .inspect(|_| trace!("Parsed config"))
            })
            .map_err(HomescreenError::from)
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
    pub async fn connect_to_database(&self) -> HomescreenResult<Data<Box<MySqlPool>>> {
        MySqlPool::connect(&self.database_url)
            .await
            .map_err(|err| {
                StartupError::CannotConnectToDatabase(err, self.database_url().to_string())
            })
            .inspect(|_| trace!("Connected to database"))
            .map(Box::new)
            .map(Data::new)
            .map_err(HomescreenError::from)
    }
}
