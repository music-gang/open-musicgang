use std::sync::{Arc, Mutex};

use openmusicgang_config::app_config::AppConfig;
use openmusicgang_postgres::{db::DB, user::UserService as PgUserService};

fn main() {
    let app = Main::new();
    app.run();
    app.close();
}

#[allow(dead_code)]
struct Main {
    config: openmusicgang_config::app_config::AppConfig,
    postgres: Arc<Mutex<DB>>,
}

impl Main {
    fn new() -> Box<Main> {
        let config = AppConfig::new("config.toml");

        Box::new(Main {
            config: AppConfig::new("config.toml"),
            postgres: Arc::new(Mutex::new(DB::new(config.get_dsn()))),
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
