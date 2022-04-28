use openmusicgang_common::error::{Error, ErrorCode};
use postgres::{Client, NoTls, Transaction};

pub mod migrations;

/// Connect to the postgres database
fn connection(dsn: &str) -> Result<Client, Error> {
    let client = Client::connect(dsn, NoTls);
    match client {
        Ok(client) => Ok(client),
        Err(error) => Err(Error::new(ErrorCode::EINTERNAL, error.to_string())),
    }
}

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

    pub fn migrate(&mut self) -> Result<(), Error> {
        if let Err(err) = self.create_migrations_table() {
            return Err(err);
        }

        let mut tx = self.begin_tx()?;

        for migration in migrations::migrations() {
            let query = format!(
                "SELECT COUNT(*) FROM migrations WHERE name = '{}'",
                migration.name
            );
            let result = tx.query_one(&query, &[]);

            if let Err(err) = result {
                return Err(Error::new(ErrorCode::EINTERNAL, err.to_string()));
            }

            let row = result.unwrap();
            let count: i64 = row.get(0);

            if count != 0 {
                continue;
            }

            let res = tx.execute(migration.query, &[]);
            if let Err(err) = res {
                return Err(Error::new(ErrorCode::EINTERNAL, err.to_string()));
            }

            let query = format!("INSERT INTO migrations (name) VALUES ($1)");

            let res = tx.execute(&query, &[&migration.name]);

            if let Err(err) = res {
                return Err(Error::new(ErrorCode::EINTERNAL, err.to_string()));
            }
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
            Ok(client) => {
                self.client = Some(client);
            }
            Err(error) => {
                return Err(error);
            }
        }

        Ok(())
    }
}

/// Module for query utilities
pub mod query_utils {
    /// Return a formatted limit offset clause.
    /// For example:
    /// format_limit_offset(10, 0) => "LIMIT 10"
    /// ```
    /// use openmusicgang_postgres::query_utils::*;
    /// let limit = 10;
    /// let offset = 0;
    /// let clause = format_limit_offset(limit, offset);
    /// assert_eq!(clause, "LIMIT 10")
    /// ```
    pub fn format_limit_offset(limit: i64, offset: i64) -> String {
        if limit > 00 && offset > 00 {
            return format!("LIMIT {} OFFSET {}", limit, offset);
        } else if limit > 00 {
            return format!("LIMIT {}", limit);
        } else if offset > 00 {
            return format!("OFFSET {}", offset);
        } else {
            return "".to_string();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_format_limit_offset() {
            let query = format_limit_offset(40, 0);
            assert_eq!(query, "LIMIT 40");

            let query = format_limit_offset(0, 40);
            assert_eq!(query, "OFFSET 40");

            let query = format_limit_offset(40, 40);
            assert_eq!(query, "LIMIT 40 OFFSET 40");

            let query = format_limit_offset(0, 0);
            assert_eq!(query, "");
        }
    }
}
