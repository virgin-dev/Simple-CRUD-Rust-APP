use crate::models::user::{User, UserListResponse};
use crate::RoleController::group_repository::RoleRepository;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

impl Role {
    pub fn get_id(&self) -> Uuid { self.id }
    pub fn get_display_name(&self) -> String { self.display_name.clone() }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_description(&self) -> String { self.description.clone() }
    pub fn get_create_timestamp(&self) -> String { self.create_timestamp.clone() }
    pub fn get_modify_timestamp(&self) -> String { self.modify_timestamp.clone() }
    pub fn get_manager(&self) -> Uuid { self.manager }
    pub fn get_members(&self) -> &Vec<Uuid> { &self.members }
    pub fn set_display_name(&mut self, display_name: String) { self.display_name = display_name; }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn set_description(&mut self, description: String) { self.description = description; }
    pub fn set_modify_timestamp(&mut self, modify_timestamp: String) { self.modify_timestamp = modify_timestamp; }
    pub fn set_manager(&mut self, manager: Uuid) { self.manager = manager; }
    pub fn set_members(&mut self, members: Vec<Uuid>) { self.members = members; }
    pub async fn get_manager_ref(&self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let manager = self.get_manager();
        let query = RoleRepository::get_manger_ref(pool, &manager).await;

        match query {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(sqlx::Error::RowNotFound),
            Err(err) => Err(err),
        }
    }
    pub async fn get_all_members(&self, pool: &PgPool) -> Result<UserListResponse, sqlx::Error> {
        let members = self.get_members();
        let users = RoleRepository::get_all_members(pool, &members).await;
        
        match users {
            Ok(users) => {
                Ok(users)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
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