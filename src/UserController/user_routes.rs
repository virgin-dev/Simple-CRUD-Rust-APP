use actix_web::{delete, put};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::{ PgPool};
use std::collections::HashMap;
use std::{env};
use log::{error, info, log};
use actix_web::get;
use actix_web::post;
use log::debug;
use actix_web::HttpRequest;
use actix_web::http::header::AUTHORIZATION;
use base64::decode;
use std::str::from_utf8;
use crate::UserService;
use crate::{CreateUser, UpdateUser};


#[post("/auth")]
async fn basic_auth_user(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let base64_encoded = &auth_str[6..];
                if let Ok(decoded_bytes) = decode(base64_encoded) {
                    if let Ok(decoded_str) = from_utf8(&decoded_bytes) {
                        let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let email = parts[0];
                            let password = parts[1];

                            let result = sqlx::query!("SELECT password FROM users WHERE email = $1", email)
                                .fetch_one(pool.get_ref())
                                .await;

                            match result {
                                Ok(record) => {
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
    HttpResponse::Unauthorized().json(json!({"error": "Authorization header required"}))
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match UserService::get_users(pool).await {
        Ok(users) => HttpResponse::Ok().json(json!({
            "users": users,
            "count": users.len()
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Failed get users",
            "reason": e.to_string()
        })),
    }
}

#[get("/users/{id}")]
async fn get_user_by_id(pool: web::Data<PgPool>,  user_id: web::Path<i32>) -> impl Responder {
    info!("Received request to get user with id: {}", user_id);

    let finded_user = UserService::get_user_by_id(pool, *user_id).await;

    match finded_user {
        Ok(response) => {
            HttpResponse::Ok().json(response)
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
pub async fn filter_users(pool: web::Data<PgPool>, query: web::Json<HashMap<String, String>>) -> HttpResponse {
    let data = query.clone();

    match UserService::get_filtered_users(pool, data).await {
        Ok(users) => {
            debug!("Finded {} users", users.len());
            HttpResponse::Ok().json(json!({
                "count": users.len(),
                "users": users
            }))
        },
        Err(e) => {
            error!("Error while getting users by filter: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Error while getting users by filter",
                "reason": e.to_string()
            }))
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
    let data_user = user.into_inner();
    match UserService::create_user(pool, data_user).await {
        Ok(response) => {
            HttpResponse::Created().json(json!({ 
                "id": response.id, 
                "name": response.name, 
                "email": response.email 
            }))
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Failed to crteate user",
            "reason": e.to_string()
        }))
    }
}