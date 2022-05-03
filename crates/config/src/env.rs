use std::fmt::Display;

use openmusicgang_common::traits::DeserializeWith;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Env {
    Local,
    Development,
    Testing,
    Staging,
    Production,
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Local => write!(f, "local"),
            Env::Development => write!(f, "development"),
            Env::Testing => write!(f, "testing"),
            Env::Staging => write!(f, "staging"),
            Env::Production => write!(f, "production"),
        }
    }
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
