use crate::models::user::{CreateUser, User};
use crate::UserController::user_service::UserService;
use actix_web::web;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn get_user_by_id(pool: web::Data<PgPool>, id: Uuid) -> Result<User, sqlx::Error> {
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
    
    pub async fn create_user(pool: web::Data<PgPool>, user: CreateUser) -> Result<User, sqlx::Error> {
        let hashed_password = UserService::hash_password(&user.password);
        let uuid = Uuid::new_v4();
        let result = sqlx::query!(
            "INSERT INTO users (name, email, password, id) VALUES ($1, $2, $3, $4) RETURNING id, name, email",
            user.name,
            user.email,
            hashed_password,
            uuid,
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

    pub async fn delete_user(pool: web::Data<PgPool>, user_id: Uuid) -> Result<HashMap<Uuid, String>,sqlx::Error> {
        let request = sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            user_id
        )
            .execute(pool.get_ref())
            .await;

        match request {
            Ok(query_request) => {
                if query_request.rows_affected() > 0 {
                    let mut result: HashMap<Uuid, String> = HashMap::new();
                    let message = format!("User with id {} deleted successfully.", user_id);
                    result.insert(user_id, message);
                    Ok(result)
                } else {
                    let mut result: HashMap<Uuid, String> = HashMap::new();
                    let message = format!("User with id {} not found.", user_id);
                    result.insert(user_id, message);
                    Ok(result)
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn update_user(pool: web::Data<PgPool>, user_id: Uuid, updates: HashMap<String, String>) -> Result<Option<Uuid>, sqlx::Error> {
        if updates.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let mut query = String::from("UPDATE users SET ");
        let mut arguments = PgArguments::default();
        let mut set_clauses = Vec::new();

        for (i, (key, value)) in updates.iter().enumerate() {
            set_clauses.push(format!("{} = ${}", key, i + 1));
            arguments.add(value);
        }

        query.push_str(&set_clauses.join(", "));
        query.push_str(" WHERE id = $");
        query.push_str(&(set_clauses.len() + 1).to_string());
        query.push_str(" RETURNING id;");
        arguments.add(user_id);

        let result = sqlx::query_with(&query, arguments)
            .fetch_optional(pool.get_ref())
            .await?;

        Ok(result.map(|row| row.get::<Uuid,_>("id")))
    }

    pub async fn save_photo(pool: &PgPool, photo_bytes: Vec<u8>, owner_oid: Uuid) -> Result<(), sqlx::Error> {
        let photo_id = Uuid::new_v4();
        sqlx::query!(
        "INSERT INTO photos (oid, photo, owner_oid) VALUES ($1, $2, $3)",
        photo_id,
        photo_bytes,
        owner_oid
    )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_photo(pool: &PgPool, owner_oid: Uuid) -> Result<Vec<u8>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT photo FROM photos WHERE owner_oid = $1",
            owner_oid
        ).fetch_one(pool)
            .await?;

        Ok(result.photo)
    }
}