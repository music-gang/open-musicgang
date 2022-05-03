use crate::migrations;
use openmusicgang_common::error::{Error, ErrorCode};
use postgres::{Client, NoTls, Transaction};

/// DB is a struct that contains the connection to the database
pub struct DB {
    client: Option<Client>,
    pub dsn: String,
}

impl DB {
    /// Create a new DB struct
    pub fn new(dsn: String) -> DB {
        DB {
            client: None,
            dsn: dsn,
        }
    }

    /// Begin a new transaction
    pub fn begin_tx(&mut self) -> Result<Transaction, Error> {
        if let Some(ref mut client) = self.client {
            match client.transaction() {
                Ok(tx) => {
                    return Ok(tx);
                }
                Err(error) => {
                    return Err(Error::new(ErrorCode::EINTERNAL, error.to_string()));
                }
            }
        } else {
            return Err(Error::new(
                ErrorCode::EINTERNAL,
                "No connection to database".to_string(),
            ));
        }
    }

    /// Create migrations table if it doesn't exist
    pub fn create_migrations_table(&mut self) -> Result<(), Error> {
        let mut tx = self.begin_tx()?;

        let query = "CREATE TABLE IF NOT EXISTS migrations (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )";

        tx.batch_execute(query)
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))
    }

    /// Migrate the database to the latest version.
    pub fn migrate(&mut self) -> Result<(), Error> {
        if let Err(err) = self.create_migrations_table() {
            return Err(err);
        }

        let mut tx = self.begin_tx()?;

        for migration in migrations::get_migrations_list() {
            let query = format!("SELECT COUNT(*) FROM migrations WHERE name = $1",);
            let row = tx
                .query_one(&query, &[&migration.name])
                .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

            let count: i64 = row.get(0);
            if count != 0 {
                continue;
            }

            tx.execute(migration.query, &[])
                .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

            let query = format!("INSERT INTO migrations (name) VALUES ($1)");

            tx.execute(&query, &[&migration.name])
                .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;
        }

        tx.commit()
            .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))
    }

    /// Connect to the database
    pub fn open(&mut self) -> Result<(), Error> {
        if self.dsn == "" {
            return Err(Error::new(ErrorCode::EINVALID, format!("No DSN provided")));
        }

        let client = connection(&self.dsn);

        match client {
            Ok(client) => self.client = Some(client),
            Err(error) => return Err(error),
        }

        Ok(())
    }

    /// Close the connection to the database, simply drops self
    pub fn close(&self) {
        drop(self);
    }
}

/// Connect to the postgres database
fn connection(dsn: &str) -> Result<Client, Error> {
    let client = Client::connect(dsn, NoTls);
    match client {
        Ok(client) => Ok(client),
        Err(error) => Err(Error::new(ErrorCode::EINTERNAL, error.to_string())),
    }
}

#[cfg(test)]
mod tests {

    use postgres::types::ToSql;

    use super::*;

    #[test]
    fn test_connection() {
        let dsn = openmusicgang_config::app_config::AppConfig::new("../../config.toml").get_dsn();
        let mut db = DB::new(dsn);
        if let Err(error) = db.open() {
            panic!("{}", error);
        }

        must_drop_table_if_exists(&mut db, "test_table");

        let mut query = "CREATE TABLE test_table (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )";
        must_exec(&mut db, query, &[]);

        query = "INSERT INTO test_table (name) VALUES ($1)";
        must_exec(&mut db, query, &[&"test"]);

        query = "SELECT * FROM test_table";
        must_exec(&mut db, query, &[]);
        must_truncate_table(&mut db, "test_table");

        db.close();
        println!("OK!");
    }

    #[allow(dead_code)]
    fn must_open_db() -> DB {
        let dsn = openmusicgang_config::app_config::AppConfig::new("../../config.toml").get_dsn();
        DB::new(dsn.to_string())
    }

    #[allow(dead_code)]
    fn must_exec(db: &mut DB, query: &str, params: &[&(dyn ToSql + Sync)]) {
        let mut tx = db.begin_tx().unwrap();

        if let Err(error) = tx.execute(query, params) {
            panic!("{}", error);
        }

        if let Err(error) = tx.commit() {
            panic!("{}", error);
        }
    }

    #[allow(dead_code)]
    fn must_truncate_table(db: &mut DB, table: &str) {
        let query = format!("TRUNCATE TABLE {}", table);
        must_exec(db, &query, &[]);
    }

    #[allow(dead_code)]
    fn must_drop_table_if_exists(db: &mut DB, table: &str) {
        let query = format!("DROP TABLE IF EXISTS {}", table);
        must_exec(db, &query, &[]);
    }
}
