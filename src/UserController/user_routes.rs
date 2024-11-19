use actix_web::dev::Response;
use actix_web::{delete, put};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use log::{error, info};
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
async fn get_user_by_id(pool: web::Data<PgPool>,  user_id: web::Path<Uuid>) -> impl Responder {
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
async fn delete_user_by_id(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> HttpResponse {
    debug!("Received request to delete user with id: {}", user_id);
    match UserService::delete_user(pool, *user_id).await {
        Ok(message) => {
            let cringe_str = format!("User with id {} not found.", user_id);
            if message.get(&user_id).expect("String is null") == &cringe_str {
                HttpResponse::NotFound().json(json!({
                    "message": message.get(&user_id).expect("String is null")
                }))
            } else {
                HttpResponse::Ok().json(json!({
                    "message": message.get(&user_id).expect("Strinf is null")
                }))
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string()
            }))
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
async fn update_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>, user_updates: web::Json<HashMap<String, String>>) -> HttpResponse {
    match UserService::update_user(pool, *user_id, user_updates.clone()).await {
        Ok(id) => {
            HttpResponse::Accepted().json(json!({
                "id": id.expect("No id found"),
                "message": "User succesfully updated"
            }))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to update user",
                "message": e.to_string()
            }))
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