use sqlx::{postgres::{PgPoolOptions, PgConnectOptions}, PgPool};
use std::time::Duration;
use tracing::debug;

pub struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    max_connections: u32,
    acquire_timeout: Duration,
    max_lifetime: Option<Duration>,
}

impl Database {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        database: &str,
        max_connections: u32,
        acquire_timeout: Duration,
        max_lifetime: Option<Duration>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            username: username.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            max_connections,
            acquire_timeout,
            max_lifetime,
        }
    }
    pub fn init_connect_options(&self) -> PgConnectOptions {
        let pg_config = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .database(&self.database);
        debug!("Database connection options: {:?}", &pg_config);
        pg_config
    }
    pub async fn create_pool(&self) -> PgPool {
        let connect_options = self.init_connect_options();
        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(self.acquire_timeout)
            .max_lifetime(self.max_lifetime)
            .connect_with(connect_options)
            .await
            .expect("Failed to create database pool");
        debug!("Database pool created with options: {:?}", &pool);
        pool
    }
}
