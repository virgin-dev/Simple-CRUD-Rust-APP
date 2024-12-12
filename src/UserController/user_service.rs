use crate::models::filter::Filter;
use crate::models::filter::Filterable;
use crate::models::user::{CreateUser, User};
use crate::UserController::user_repository::UserRepository;
use actix_web::web;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

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