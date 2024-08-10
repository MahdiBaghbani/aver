use config::{Config, ConfigError, Environment, File};
use std::env;

use crate::settings::models::{Server, Settings};
use crate::settings::SETTINGS;

pub fn settings() -> &'static Settings {
    SETTINGS.get().expect("config init")
}

pub fn init() {
    let settings: Settings = Settings::new().unwrap();
    SETTINGS.set(settings).expect("Somehow Darth Sidious has returned!");
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let file_path: String = env::var("AVER_CONFIG_PATH").unwrap();

        let settings: Config = Config::builder()
            .add_source(File::with_name(&file_path))
            // add in settings from the environment (with a prefix of APP)
            // eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("AVER"))
            .build()?;

        // deserialize and thus freeze the entire configuration.
        settings.try_deserialize()
    }
}

impl Server {
    pub fn get_tcp_bind(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
