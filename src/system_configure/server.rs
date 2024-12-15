use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub backlog: u32,
    pub max_connections: usize,
    pub keep_alive: u64,
    pub client_timeout: u64,
    pub client_disconnect_timeout: u64
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub acquire_timeout_secs: u64,
    pub acquire_timeout_nanos: u32,
    pub max_lifetime_secs: u64,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("_"));
        let config = builder.build().expect("Failed to build configuration");
        config.try_deserialize().expect("Failed to deserialize configuration")
    }
}