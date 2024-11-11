use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sqlx::{pool, PgPool};
use std::env;
use log::{info, error};


#[derive(Deserialize, Debug)]
struct CreateUser {
    name: String,
    email: String,
}
#[derive(Serialize, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

async fn create_user(pool: web::Data<PgPool>, user: web::Json<CreateUser>) -> HttpResponse {
    info!("Received request to create user: {:?}", user);
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        user.name,
        user.email
    ).fetch_one(pool.get_ref()).await;
    
    match result {
        Ok(record) => {
            info!("User created successfully: {:?}", record);
            HttpResponse::Created().json(json!({"id": record.id}))
        },
        Err(e) => {
            error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    info!("Received reQUEST TO GET ALL users");

    let result = sqlx::query!(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            let users: Vec<User> = records.into_iter()
                .map(|record| User {
                    id: record.id,
                    name: record.name,
                    email: record.email,
                })
                .collect();
            info!("Users retrived seccesfully: {:?}", users);
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            error!("Error retrived users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_user_by_id(pool: web::Data<PgPool>,  user_id: web::Path<i32>) -> impl Responder {
    info!("Received request to get user with id: {}", user_id);

    let result = sqlx::query!(
        "SELECT id, name, email FROM users WHERE id = $1",
        *user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let user = User {
                id: record.id,
                name: record.name,
                email: record.email,
            };
            info!("User retrieved successfully: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            error!("Error retrieving user with id {}: {}", user_id, e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    //println!("Using database URL: {}", database_url);
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}