use config::{Config, File};
use openmusicgang_err::error::{Error, ErrorCode};
use serde::Deserialize;

use crate::env::Env;
use openmusicgang_app::traits::DeserializeWith;

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    #[serde(deserialize_with = "Env::deserialize_with")]
    pub env: Env,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app: App,
    pub postgres: Postgres,
    pub redis: Redis,
}

impl AppConfig {
    pub fn new(config_file_path: &str) -> Self {
        let s = Config::builder()
            .add_source(File::with_name(config_file_path))
            .build()
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, format!("{}", error)));

        if let Err(error) = s {
            panic!("{}", error);
        }

        let app_config = s
            .unwrap()
            .try_deserialize()
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, format!("{}", error)));

        if let Err(error) = app_config {
            panic!("{}", error);
        }

        app_config.unwrap()
    }

    /// Returns the database connection string.
    pub fn get_postgres_dsn(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres.username,
            self.postgres.password,
            self.postgres.host,
            self.postgres.port,
            self.postgres.database
        )
    }

    /// Returns the Redis connection string.
    pub fn get_redis_dsn(&self) -> String {
        format!(
            "redis://{}@{}:{}",
            self.redis.password, self.redis.host, self.redis.port
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_config() {
        let cfg = AppConfig::new("../../config.toml");
        assert!(
            cfg.app.env == Env::Local
                || cfg.app.env == Env::Development
                || cfg.app.env == Env::Testing
        );
    }
}
