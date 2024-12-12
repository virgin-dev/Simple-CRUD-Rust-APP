use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::group::{CreateRole, RoleResponse};

pub struct RoleController;

impl RoleController {
    pub async fn create_role(pool: web::Data<PgPool>, role: CreateRole) -> Result<RoleResponse, sqlx::Error> {
        let role_id = Uuid::new_v4();
        let result = sqlx::query!("INSERT INTO roles (id, display_name, name, description, manager) 
        VALUES ($1, $2, $3, $4, $5) RETURNING id, display_name, name, description, manager",
        role_id,
        role.display_name,
        role.name,
        role.description,
        role.manager
        )
            .fetch_one(pool.get_ref()).await;
        match result { 
            Ok(record) => {
                Ok(RoleResponse {
                    id: record.id,
                    display_name: record.display_name,
                    name: record.name,
                    description: record.description.expect("REASON"),
                    manager: record.manager.expect("REASON"),
                })
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}