use config::{File, FileFormat};

pub fn get_config() -> ApplicationSettings {
    let config_result = config::Config::builder()
        .add_source(File::new("settings.yml", FileFormat::Yaml))
        .build();
    let config = config_result.ok().expect("");
    ApplicationSettings {
        redis_url: config.get_string("redis-url").ok().expect("setting not found"),
        rabbitmq_host: config.get_string("rabbitmq-host").ok().expect("setting not found"),
        rabbitmq_port: config.get_int("rabbitmq-port").ok().expect("setting not found"),
    }
}

pub struct ApplicationSettings {
    pub redis_url: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: i64,
}
