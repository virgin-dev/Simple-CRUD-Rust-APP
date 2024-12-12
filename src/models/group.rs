use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Deserialize, Serialize, Debug)]
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
pub struct UpdateRole {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub manager: Option<Uuid>,
    pub members: Option<Vec<Uuid>>
}
pub struct RoleListResponse {
    pub count: usize,
    pub roles: Vec<Role>
}
pub struct CreateRole {
    pub display_name: String,
    pub description: String,
    pub name: String,
    pub manager: Uuid,
}
pub struct RoleResponse {
    pub id: Uuid,
    pub display_name: String,
    pub name: String,
    pub description: String,
    pub manager: Uuid,
}