use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{ PgPool};
use std::{env};
mod user_service;
use user_service::UserService;
use user_service::{CreateUser, UpdateUser, User, UserListResponse};
mod user_routes;
use user_routes::{create_user,get_users, get_users_by_name, get_user_by_id,update_user,delete_user_by_id,register_user,basic_auth_user};

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
            .service(create_user)
            .service(get_users)
            .service(get_users_by_name)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user_by_id)
            .service(register_user)
            .service(basic_auth_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}