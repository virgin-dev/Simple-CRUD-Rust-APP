use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::env;

mod UserController {
    pub mod user_routes;
    pub mod user_service;
    pub mod user_repository;
}

mod RoleController {
    pub mod group_repository;
    pub mod role_service;
    pub mod role_routes;
}

mod models {
    pub mod filter;
    pub mod user;
    pub mod group;
}

use UserController::user_routes::*;
use UserController::user_service::*;
use crate::RoleController::role_routes::{create_role, get_role};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("Using database URL: {}", database_url);
    let pool = PgPool::connect(&database_url).await.unwrap();
    HttpServer::new(move || {
        App::new()
        .wrap(Cors::default().allowed_origin("http://127.0.0.1:3000"))
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
            .service(filter_users)
            .service(get_user_by_id)
            .service(update_user)
            .service(upload_photo)
            .service(delete_user_by_id)
            .service(register_user)
            .service(basic_auth_user)
            .service(get_photo)
            .service(create_role)
            .service(get_role)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}