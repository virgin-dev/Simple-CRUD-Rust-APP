use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::group::{CreateRole, RoleResponse};

pub struct RoleController;

impl RoleController {
    pub async fn create_role(pool: web::Data<PgPool>, role: CreateRole) -> Result<RoleResponse, sqlx::Error> {
        let role_id = Uuid::new_v4();
        let query = sqlx::query_as!(RoleResponse, 
        "INSERT INTO roles (id, display_name, name, description, manager)
        VALUES ($1, $2, $3, $4, $5) RETURNING id, display_name, name, description, manager",
        role_id,
        role.display_name,
        role.name,
        role.description,
        role.manager
        ).fetch_one(pool.get_ref()).await;
        
        match query {
            Ok(record) => {
                Ok(record)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}