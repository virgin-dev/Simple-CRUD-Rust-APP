use crate::models::filter::{Filter, Filterable};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UserListResponse {
    pub count: i64,
    pub users: Vec<User>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
#[async_trait::async_trait]
impl Filterable for User {
    type Entity = User;

    async fn filter(pool: &PgPool, filter: Filter) -> Result<Vec<Self::Entity>, sqlx::Error> {
        let mut query = String::from("SELECT id, name, email FROM users WHERE 1 = 1");
        let mut arguments = PgArguments::default();

        for (i, (key, value)) in filter.fields.iter().enumerate() {
            match key.as_str() {
                "name" => {
                    query.push_str(&format!(" AND name ILIKE ${}", i + 1));
                    arguments.add(format!("%{}%", value));
                }
                "email" => {
                    query.push_str(&format!(" AND email ILIKE ${}", i + 1));
                    arguments.add(format!("%{}%", value));
                }
                "id" => {
                    query.push_str(&format!(" AND id = ${}", i + 1));
                    arguments.add(value.parse::<i32>().expect("Error parse value"));
                }
                _ => {}
            }
        }

        let rows = sqlx::query_with(&query, arguments)
            .fetch_all(pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            })
            .collect())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
impl User {

    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
        }
    }
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
    //Сеттеры
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}