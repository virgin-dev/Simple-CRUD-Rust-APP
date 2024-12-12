use crate::models::filter::Filter;
use crate::models::filter::Filterable;
use crate::UserController::user_repository::UserRepository;
use actix_web::web;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {

    pub fn new(name: String, email: String, password: String) -> Self {
        let hashed_password = UserService::hash_password(&password);
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
        UserRepository::create_user(pool, user).await
    }
    
    pub async fn get_user_by_id(pool: web::Data<PgPool>, id: Uuid) -> Result<User, sqlx::Error> {
        UserRepository::get_user_by_id(pool, id).await
    }

    pub async fn get_users(pool: web::Data<PgPool>) -> Result<Vec<User>, sqlx::Error> {
        UserRepository::get_users(pool).await
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

    pub async fn delete_user(pool: web::Data<PgPool>, user_id: Uuid) -> Result<HashMap<Uuid, String>,sqlx::Error> {
        UserRepository::delete_user(pool, user_id).await
    }

    pub async fn update_user(pool: web::Data<PgPool>, user_id: Uuid, updates: HashMap<String, String>) -> Result<Option<Uuid>, sqlx::Error> {
        UserRepository::update_user(pool, user_id, updates).await
    }

    pub async fn save_photo_to_db(pool: &PgPool, photo_bytes: Vec<u8>, owner_oid: Uuid) -> Result<(), sqlx::Error> {
        UserRepository::save_photo(pool, photo_bytes, owner_oid).await
    }

    pub async fn get_photo(pool: &PgPool, owner_oid: Uuid) -> Result<Vec<u8>, sqlx::Error> {
        UserRepository::get_photo(pool, owner_oid).await
    }
}