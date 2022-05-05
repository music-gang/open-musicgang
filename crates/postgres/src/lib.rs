pub mod migrations;
pub mod postgres;
pub mod query;
pub mod user;

#[cfg(test)]
pub mod test_utils {
    use postgres::types::ToSql;

    use crate::postgres::DB;

    #[allow(dead_code)]
    pub fn must_open_db() -> DB {
        let dsn = openmusicgang_config::app_config::AppConfig::new("../../config.toml")
            .get_postgres_dsn();
        let mut db = DB::new(dsn.to_string());
        db.open().unwrap();
        db
    }

    #[allow(dead_code)]
    pub fn must_exec(db: &mut DB, query: &str, params: &[&(dyn ToSql + Sync)]) {
        let mut tx = db.begin_tx().unwrap();

        if let Err(error) = tx.execute(query, params) {
            panic!("{}", error);
        }

        if let Err(error) = tx.commit() {
            panic!("{}", error);
        }
    }

    #[allow(dead_code)]
    pub fn must_truncate_table(db: &mut DB, table: &str) {
        let query = format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE", table);
        must_exec(db, &query, &[]);
    }

    #[allow(dead_code)]
    pub fn must_drop_table_if_exists(db: &mut DB, table: &str) {
        let query = format!("DROP TABLE IF EXISTS {}", table);
        must_exec(db, &query, &[]);
    }
}
