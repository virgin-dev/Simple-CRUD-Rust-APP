
use argon2::password_hash::{SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use argon2::password_hash::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use actix_web::{web};
use sqlx::{ PgPool};
use crate::models::filter::Filterable;
use crate::models::filter::Filter;
use sqlx::Row;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {

    pub fn new(name: String, email: String, password: String) -> Self {
        let hashed_password = UserService::hash_password(&password);
        Self {
            id: 0, 
            name,
            email,
        }
    }
    //Геттеры
    pub fn get_id(&self) -> i32 {
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

/*
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub single_use: bool,
}

impl Claims {
    pub fn new(user_id: i32, single_use: bool) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::minutes(5))
            .unwrap()
            .timestamp() as usize;
        Claims {
            sub: user_id.to_string(),
            exp,
            single_use,
        }
    }
}*/

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
pub struct UserService;

impl UserService {
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
    }
    
    pub fn verify_password(hash: &str, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }

    pub async fn create_user(pool: web::Data<PgPool>, user: CreateUser) -> Result<User, sqlx::Error> {
        let hashed_password = UserService::hash_password(&user.password);

        let result = sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email",
            user.name,
            user.email,
            hashed_password,
        )
        .fetch_one(pool.get_ref())
        .await;

        match result {
            Ok(record) => {
                log::info!("User created successfully: {:?}", record);
                Ok(User {
                    id: record.id,
                    name: record.name,
                    email: record.email,
                })
            }
            Err(e) => {
                log::error!("Error creating user: {}", e);
                Err(e)
            }
        }
    }
    
    pub async fn get_user_by_id(pool: web::Data<PgPool>, id: i32) -> Result<User, sqlx::Error> {
        let query = sqlx::query!(
            "SELECT id, name, email FROM users WHERE id = $1",
            id
        )
        .fetch_one(pool.get_ref())
        .await;

        match query {
            Ok(record) => {
                let user = User {
                    id: record.id,
                    name: record.name,
                    email: record.email,
                };
                log::info!("User retrieved successfully: {:?}", user);
                Ok(user)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn get_users(pool: web::Data<PgPool>) -> Result<Vec<User>, sqlx::Error> {
        let query_result = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users"
        )
        .fetch_all(pool.get_ref())
        .await;

        match query_result {
            Ok(users) => {
                log::info!("Fetched {} users", users.len());
                Ok(users)
            }
            Err(e) => {
                log::error!("Error fetching users: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_filtered_users(pool: web::Data<PgPool>, data: HashMap<String, String>) -> Result<Vec<User>, sqlx::Error> {
        let mut filter = Filter::new();
        for (key, value) in data.iter() {
            filter = filter.add_filter(key, value);
        }

        match User::filter(pool.get_ref(), filter).await {
            Ok(users) => {
                Ok(users)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}