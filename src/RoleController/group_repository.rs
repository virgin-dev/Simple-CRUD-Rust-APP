use crate::models::group::{AssignRoleToUser, CreateRole, ResultRoleAssign, RoleResponse, UserRolesResponse};
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::user::{User, UserListResponse};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create_role(pool: &PgPool, role: CreateRole) -> Result<RoleResponse, sqlx::Error> {
        let role_id = Uuid::new_v4();
        let query = sqlx::query_as!(RoleResponse, 
        "INSERT INTO roles (id, display_name, name, description, manager, members)
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id, display_name, name, description, manager, members",
        role_id,
        role.display_name,
        role.name,
        role.description,
        role.manager,
        &Vec::<Uuid>::new()
        ).fetch_one(pool).await;
        
        match query {
            Ok(record) => {
                Ok(record)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn update_role_attribute(pool: &PgPool, role_id: &Uuid, updates: HashMap<String, String>) -> Result<Option<Uuid>, sqlx::Error> {
        let mut query = String::from("UPDATE roles SET ");
        let mut arguments = PgArguments::default();
        let mut set_clauses = Vec::new();

        for (i, (key, value)) in updates.iter().enumerate() {
            set_clauses.push(format!("{} = ${}", key, i + 1));
            arguments.add(value);
        }

        query.push_str(&set_clauses.join(", "));
        query.push_str(" WHERE id = $");
        query.push_str(" RETURNING id");
        query.push_str(";");
        arguments.add(role_id);
        let result = sqlx::query_with(&query, arguments)
            .fetch_optional(pool)
            .await?;

        Ok(result.map(|row| row.get::<Uuid,_>("id")))
    }
    pub async fn assign_role_to_user(pool: &PgPool, data: AssignRoleToUser) -> Result<ResultRoleAssign, sqlx::Error> {
        let query = sqlx::query_as!(
                ResultRoleAssign,
                "UPDATE roles
                 SET members = array_append(members, $1), modify_timestamp = CURRENT_TIMESTAMP
                 WHERE id = $2 AND NOT ($1 = ANY(members))
                 RETURNING $1 AS user_id, 
                           $2 AS role_id,
                           (SELECT name FROM users WHERE id = $1 LIMIT 1) AS user_name, 
                           (SELECT email FROM users WHERE id = $1 LIMIT 1) AS user_email,
                           members,
                           display_name",
                data.user_id,
                data.role_id
            )
                        .fetch_one(pool)
                        .await;
        match query { 
            Ok(record) => Ok(record),
            Err(err) => Err(err)
        }
    }
    pub async fn get_roles_for_user(pool: &PgPool, user_id: &Uuid) -> Result<Option<UserRolesResponse>, sqlx::Error> {
        let roles = sqlx::query_as!(RoleResponse,
        "SELECT id, display_name, name, description, manager, members
         FROM roles
         WHERE $1 = ANY(members)",
            user_id
        ).fetch_all(pool).await?;
        let count = roles.len();

        if roles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(UserRolesResponse{count, roles: roles}))
        }
    }
    pub async fn get_all_roles(pool: &PgPool) -> Result<Vec<RoleResponse>, sqlx::Error> {
        let rows = sqlx::query_as!(
        RoleResponse,
        "SELECT id, display_name, name, description, manager, members
        FROM roles"
    )
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
    pub async fn revoke_role_from_user(pool: &PgPool, role_id: &Uuid, user_id: &Uuid, ) -> Result<(), sqlx::Error> {
        sqlx::query!(
        "UPDATE roles 
         SET members = array_remove(members, $1), modify_timestamp = CURRENT_TIMESTAMP 
         WHERE id = $2",
        user_id,
        role_id
    ).execute(pool)
     .await?;
        Ok(())
    }
    pub async fn get_role(pool: &PgPool, role_id: &Uuid) -> Result<RoleResponse, sqlx::Error> {
        let query = sqlx::query_as!(RoleResponse,
        "SELECT id, display_name, name, description, manager, members
        FROM roles
        WHERE id = $1",
        role_id).fetch_one(pool).await;
        match query {
            Ok(role) => Ok(role),
            Err(err) => Err(err)
        }
    }
    pub async fn get_manger_ref(pool: &PgPool, manager: &Uuid) -> Result<Option<User>, sqlx::Error> {
        let manager = sqlx::query_as!(User,
        "SELECT id, name, email FROM users WHERE id = $1",
            manager
        ).fetch_one(pool).await;
        match manager {
            Ok(manager) => Ok(Some(manager)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_all_members(pool: &PgPool, members: &Vec<Uuid>) -> Result<UserListResponse, sqlx::Error> {
        let users = sqlx::query_as!(User,
        "SELECT id, name, email FROM users WHERE id = ANY($1)",
            members
        ).fetch_all(pool).await;
        match users {
            Ok(users) => {
                let count = users.len() as i64;
                Ok(UserListResponse { count, users })
            },
            Err(err) => Err(err),
        }
    }
}