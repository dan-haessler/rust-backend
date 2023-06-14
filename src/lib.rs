use config::ConfigError;
use serde::Deserialize;

pub mod api;
pub mod database;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub listen: String,
  pub workers: usize,
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    config::Config::builder()
      .add_source(config::Environment::default())
      .build()
      .unwrap()
      .try_deserialize()
  }
}
