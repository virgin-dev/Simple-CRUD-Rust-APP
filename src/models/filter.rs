use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
#[derive(Debug)]
pub struct Filter {
    pub fields: HashMap<String, String>,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            fields: HashMap::new(),
        }
    }

    pub fn add_filter(mut self, key: &str, value: &str) -> Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

#[async_trait]
pub trait Filterable {
    type Entity;

    async fn filter(pool: &PgPool, filter: Filter) -> Result<Vec<Self::Entity>, sqlx::Error>;
}