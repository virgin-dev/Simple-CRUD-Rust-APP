
use argon2::password_hash::{SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use argon2::password_hash::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;
use base64::decode;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserListResponse {
    pub count: i64,
    pub users: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub single_use: bool,
}

impl Claims {
    pub fn new(user_id: i32, single_use: bool) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::minutes(5))
            .unwrap()
            .timestamp() as usize;
        Claims {
            sub: user_id.to_string(),
            exp,
            single_use,
        }
    }
}

pub struct UserService;

impl UserService {
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
    }
    
    pub fn verify_password(hash: &str, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }

    // Other user-related methods (e.g., creating, updating users) would go here
}
