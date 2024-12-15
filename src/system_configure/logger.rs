use std::fs::OpenOptions;
use tracing::{debug, info, warn, error};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

pub struct Logger {
    log_file: String,
    level: String,
}

impl Logger {
    pub fn new(log_file: &str, level: &str) -> Self {
        Self {
            log_file: log_file.to_string(),
            level: level.to_string(),
        }
    }

    pub fn init(&self) {
        let console_layer = fmt::layer()
            .pretty()
            .with_target(true)
            .with_level(true);

        let file_appender = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file)
            .expect("Failed to open log file");
        
        debug!("Log file appender path: {:?}", self.log_file);

        let file_layer = fmt::layer()
            .with_writer(file_appender)
            .with_ansi(false)
            .with_line_number(true)
            .with_target(true)
            .with_level(true);

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.level));

        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .with(filter)
            .init();

        info!("Logger initialized with level: {}", self.level);
    }
}
