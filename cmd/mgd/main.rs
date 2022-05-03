use openmusicgang_config::app_config::AppConfig;

fn main() {
    let app = App::new();

    println!("current env: {}", app.config.app.env);
}

#[allow(dead_code)]
struct App {
    pub config: openmusicgang_config::app_config::AppConfig,
}

impl App {
    fn new() -> Box<App> {
        Box::new(App {
            config: AppConfig::new("config.toml"),
        })
    }
}
