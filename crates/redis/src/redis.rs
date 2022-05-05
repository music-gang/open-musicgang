use std::sync::Mutex;

use once_cell::sync::Lazy;
use openmusicgang_err::error::{Error, ErrorCode};
use redis::{Client, Connection};

/// The Resource is used to manage mutex on shared resources.
static THE_RESOURCE: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

// REDIS_CMD_PING is the command used to ping the redis server
#[allow(dead_code)]
static REDIS_CMD_PING: &str = "PING";

pub struct DB {
    conn: Option<Connection>,
    dsn: String,
}

impl DB {
    pub fn new(dsn: String) -> DB {
        DB {
            conn: None,
            dsn: dsn,
        }
    }

    pub fn open(&mut self) -> Result<(), Error> {
        if self.dsn == "" {
            return Err(Error::new(ErrorCode::EINVALID, format!("No DSN provided")));
        }

        let _shared = THE_RESOURCE.lock();

        let client = Client::open(self.dsn.as_str())
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;
        let mut conn = client
            .get_connection()
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

        redis::cmd(REDIS_CMD_PING)
            .query::<()>(&mut conn)
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

        self.conn = Some(conn);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_connection() {
        let dsn =
            openmusicgang_config::app_config::AppConfig::new("../../config.toml").get_redis_dsn();
        let mut db = DB::new(dsn);
        if let Err(error) = db.open() {
            panic!("{}", error);
        }
    }
}
