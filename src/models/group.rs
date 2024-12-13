use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Role {
    pub id: Uuid,
    pub display_name: String,
    pub name: String,
    pub description: String,
    pub create_timestamp: String,
    pub modify_timestamp: String,
    pub manager: Uuid,
    pub members: Vec<Uuid>
}
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UpdateRole {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub manager: Option<Uuid>,
    pub members: Option<Vec<Uuid>>
}
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct RoleListResponse {
    pub count: usize,
    pub roles: Vec<Role>
}
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct CreateRole {
    pub display_name: String,
    pub description: Option<String>,
    pub name: String,
    pub manager: Option<Uuid>,
    pub members: Option<Vec<Uuid>>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct RoleResponse {
    pub id: Uuid,
    pub display_name: String,
    pub name: String,
    pub description: Option<String>,
    pub manager: Option<Uuid>,
    pub members: Option<Vec<Uuid>>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UserRolesResponse {
    pub count: usize,
    pub roles: Vec<RoleResponse>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct ResultRoleAssign {
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub members: Option<Vec<Uuid>>,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignRoleToUser {
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
}