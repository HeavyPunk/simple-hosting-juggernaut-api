use config::{File, FileFormat};

pub fn get_config() -> ApplicationSettings {
    let config_result = config::Config::builder()
        .add_source(File::new("settings.yml", FileFormat::Yaml))
        .build();
    let config = config_result.ok().expect("");
    ApplicationSettings {
        redis_url: config.get_string("redis-url").ok().expect("setting not found"),
        rabbitmq_connection_string: config.get_string("rabbitmq-connection-string").ok().expect("setting not found"),
    }
}

pub struct ApplicationSettings {
    pub redis_url: String,
    pub rabbitmq_connection_string: String,
}
