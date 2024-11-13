use actix_web::{delete, put};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::{ PgPool};
use std::collections::HashMap;
use std::{env};
use log::{info, error};
use actix_web::get;
use actix_web::post;
use log::debug;
use actix_web::HttpRequest;
use actix_web::http::header::AUTHORIZATION;
use base64::decode;
use std::str::from_utf8;
use crate::UserService;
use crate::{CreateUser, UpdateUser, User, UserListResponse};

#[post("/auth")]
async fn basic_auth_user(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    // Получаем заголовок авторизации
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            // Проверяем, что заголовок начинается с "Basic "
            if auth_str.starts_with("Basic ") {
                // Извлекаем Base64-кодированную строку
                let base64_encoded = &auth_str[6..];
                
                // Декодируем Base64-строку
                if let Ok(decoded_bytes) = decode(base64_encoded) {
                    // Преобразуем байты в строку
                    if let Ok(decoded_str) = from_utf8(&decoded_bytes) {
                        // Разделяем строку по первому символу ':'
                        let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let email = parts[0];
                            let password = parts[1];

                            // Проверяем наличие пользователя в базе данных
                            let result = sqlx::query!("SELECT password FROM users WHERE email = $1", email)
                                .fetch_one(pool.get_ref())
                                .await;

                            match result {
                                Ok(record) => {
                                    // Проверяем, что пароль совпадает
                                    if UserService::verify_password(&record.password, password) {
                                        info!("Authentication successful for user: {}", email);
                                        return HttpResponse::Ok().json(json!({"message": "Authentication successful"}));
                                    } else {
                                        error!("Invalid password for user: {}", email);
                                        return HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}));
                                    }
                                },
                                Err(_) => {
                                    error!("User not found: {}", email);
                                    return HttpResponse::Unauthorized().json(json!({"error": "User not found"}));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Если что-то не так с заголовком авторизации
    HttpResponse::Unauthorized().json(json!({"error": "Authorization header required"}))
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<CreateUser>) -> HttpResponse {
    info!("Received request to create user: {:?}", user);
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        user.name,
        user.email
    ).fetch_one(pool.get_ref()).await;
    
    match result {
        Ok(record) => {
            info!("User created successfully: {:?}", record);
            HttpResponse::Created().json(json!({"id": record.id}))
        },
        Err(e) => {
            error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    info!("Received reQUEST TO GET ALL users");

    let result = sqlx::query!(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            let users: Vec<User> = records.into_iter()
                .map(|record| User {
                    id: record.id,
                    name: record.name,
                    email: record.email,
                })
                .collect();
            info!("Users retrived seccesfully: {:?}", users);
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            error!("Error retrived users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users/{id}")]
async fn get_user_by_id(pool: web::Data<PgPool>,  user_id: web::Path<i32>) -> impl Responder {
    info!("Received request to get user with id: {}", user_id);

    let result = sqlx::query!(
        "SELECT id, name, email FROM users WHERE id = $1",
        *user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let user = User {
                id: record.id,
                name: record.name,
                email: record.email,
            };
            info!("User retrieved successfully: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            error!("Error retrieving user with id {}: {}", user_id, e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[delete("/users/{id}")]
async fn delete_user_by_id(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> HttpResponse {
    debug!("Received request to delete user with id: {}", user_id);

    let request = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        *user_id
    )
    .execute(pool.get_ref())
    .await;

    match request {
        Ok(query_request) => {
            if query_request.rows_affected() > 0 {
                debug!("User with id {} deleted successfully.", user_id);
                HttpResponse::Ok().finish()
            } else {
                debug!("User with id {} not found.", user_id);
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Error deleting user with id {}: {}", user_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users/search")]
async fn get_users_by_name(pool: web::Data<PgPool>, web::Query(params): web::Query<HashMap<String, String>>) -> impl Responder {
    info!("Received request to search users by name");

    let name = params.get("name").map(|s| s.as_str()).unwrap_or("");
    let limit = params.get("limit").and_then(|s| s.parse::<i64>().ok()).unwrap_or(10);
    let offset = params.get("offset").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

    let records = sqlx::query!(
        "SELECT id, name, email FROM users WHERE name ILIKE $1 LIMIT $2 OFFSET $3",
        format!("%{}%", name),
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await;

    let total_count_res = sqlx::query!(
        "SELECT COUNT(*) AS count FROM users WHERE name ILIKE $1",
        format!("%{}%", name)
    )
    .fetch_one(pool.get_ref())
    .await;

    let total_count = match total_count_res {
        Ok(record) => record.count.unwrap_or(0), 
        Err(_) => 0, 
    };

    match records {
        Ok(user_records) => {
            let users: Vec<User> = user_records.into_iter()
                .map(|record| User {
                    id: record.id,
                    name: record.name,
                    email: record.email,
                })
                .collect();
            let response = UserListResponse {
                count: total_count,
                users,
            };
            info!("Users found: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("Error searching users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/users/{id}")]
async fn update_user(pool: web::Data<PgPool>, user_id: web::Path<i32>, user_updates: web::Json<UpdateUser>) -> HttpResponse {
    let result = sqlx::query!(
        "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3",
        user_updates.name.as_deref(),
        user_updates.email.as_deref(),
        *user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            info!("User with id {} updated successfully.", user_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("Error updating user with id {}: {}", user_id, e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("/register")]
async fn register_user(pool: web::Data<PgPool>, user: web::Json<CreateUser>) -> HttpResponse {
    let hashed_password = UserService::hash_password(&user.password);
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id",
        user.name,
        user.email,
        hashed_password,
    ).fetch_one(pool.get_ref()).await;

    match result {
        Ok(record) => {
            info!("User created successfully: {:?}", record);
            HttpResponse::Created().json(json!({"id": record.id}))
        },
        Err(e) => {
            error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}