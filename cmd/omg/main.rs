use std::sync::{Arc, Mutex};

use openmusicgang_config::app_config::AppConfig;
use openmusicgang_postgres::{db::DB as PgDB, user::UserService as PgUserService};
use openmusicgang_redis::redis::DB as RedisDB;

fn main() {
    let app = Main::new();
    app.run();
    app.close();
}

#[allow(dead_code)]
struct Main {
    config: openmusicgang_config::app_config::AppConfig,
    postgres: Arc<Mutex<PgDB>>,
    redis: Arc<Mutex<RedisDB>>,
}

impl Main {
    fn new() -> Box<Main> {
        let config = AppConfig::new("config.toml");

        Box::new(Main {
            config: AppConfig::new("config.toml"),
            postgres: Arc::new(Mutex::new(PgDB::new(config.get_postgres_dsn()))),
            redis: Arc::new(Mutex::new(RedisDB::new(config.get_redis_dsn()))),
        })
    }

    #[allow(dead_code)]
    fn close(&self) {
        drop(self);
    }

    fn run(&self) {
        let _postgres_user_service = PgUserService::new(self.postgres.clone());

        println!("current env: {}", self.config.app.env);
    }
}
