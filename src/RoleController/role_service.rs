use crate::models::group::{AssignRoleToUser, CreateRole, ResultRoleAssign, RoleResponse, UserRolesResponse};
use crate::RoleController::group_repository::RoleRepository;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct RoleService;

impl RoleService {
    pub async fn create_role(pool: &PgPool, role: CreateRole) -> Result<RoleResponse, sqlx::Error> {
        RoleRepository::create_role(pool, role).await
    }
    pub async fn update_role_attribute(pool: &PgPool, role_id: &Uuid, updates: HashMap<String, String>) -> Result<Option<Uuid>, sqlx::Error> {
        RoleRepository::update_role_attribute(pool, role_id, updates).await
    }
    pub async fn assign_role_to_user(pool: &PgPool, data: AssignRoleToUser) -> Result<ResultRoleAssign, sqlx::Error> {
        RoleRepository::assign_role_to_user(pool, data).await
    }
    pub async fn get_roles_for_user(pool: &PgPool, user_id: &Uuid) -> Result<Option<UserRolesResponse>, sqlx::Error> {
        RoleRepository::get_roles_for_user(pool, user_id).await
    }
    pub async fn get_all_roles(pool: &PgPool) -> Result<Vec<RoleResponse>, sqlx::Error> {
        RoleRepository::get_all_roles(pool).await
    }
    pub async fn revoke_role_from_user(pool: &PgPool, role_id: &Uuid, user_id: &Uuid, ) -> Result<(), sqlx::Error> {
        RoleRepository::revoke_role_from_user(pool, role_id, user_id).await
    }
    pub async fn get_role(pool: &PgPool, role_id: &Uuid) -> Result<RoleResponse, sqlx::Error> {
        RoleRepository::get_role(pool, role_id).await
    }
}