use config::{Config, File};
use openmusicgang_common::error::{Error, ErrorCode};
use serde::{Deserialize, Deserializer};

pub trait DeserializeWith: Sized {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Env {
    Local,
    Development,
    Testing,
    Staging,
    Production,
}

impl DeserializeWith for Env {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(de)?;

        match s.as_ref() {
            "local" => Ok(Env::Local),
            "development" => Ok(Env::Development),
            "testing" => Ok(Env::Testing),
            "staging" => Ok(Env::Staging),
            "production" => Ok(Env::Production),
            _ => Ok(Env::Local),
        }
    }
}

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
pub struct AppConfig {
    pub app: App,
    pub postgres: Postgres,
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_config() {
        let cfg = AppConfig::new("../../config");
        assert!(
            cfg.app.env == Env::Local
                || cfg.app.env == Env::Development
                || cfg.app.env == Env::Testing
        );
    }
}
