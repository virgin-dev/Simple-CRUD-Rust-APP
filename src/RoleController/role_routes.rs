use crate::models::group::CreateRole;
use crate::RoleController::role_service::RoleService;
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[post("/role/create")]
async fn create_role(pool: web::Data<PgPool>, role: web::Json<CreateRole>) -> HttpResponse {
    let role = role.into_inner();
    let pool = pool.get_ref();
    let create_role = RoleService::create_role(pool, role).await;
    match create_role { 
        Ok(role) => {
            HttpResponse::Created().json(json!({
                "id": role.id,
                "display_name": role.display_name,
                "name": role.name,
                "description": role.description,
                "manager": role.manager,
            }))
        },
        Err(err) => {HttpResponse::InternalServerError().json(json!({
            "error": "Failed to create role",
            "reason": err.to_string()
        }))}
    }
}
#[post("/role/{role_id}/update")]
async fn update_role_attributes(pool: web::Data<PgPool>, role_id: web::Path<Uuid>, updates: web::Json<HashMap<String, String>>) -> HttpResponse {
    let role_id = role_id.into_inner();
    let pool = pool.get_ref();
    
    let result = RoleService::update_role_attribute(pool, &role_id, updates.into_inner()).await;
    
    match result {
        Ok(role_id ) => {
            HttpResponse::Ok().json(json!({
                "id": role_id,
                "message": "role attributes successfully updated"
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to update role attributes",
                "reason": err.to_string()
            }))
        }
    }
}
#[get("/role/{role_id}")]
async fn get_role(pool: web::Data<PgPool>, role_id: web::Path<Uuid>) -> HttpResponse {
    let role_id = role_id.into_inner();
    let pool = pool.get_ref();
    
    let result = RoleService::get_role(pool, &role_id).await;
    
    match result {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => {HttpResponse::InternalServerError().json(json!({
            "error": "Failed to get role",
            "reason": err.to_string()
        }))}
    }
}