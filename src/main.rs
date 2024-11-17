use actix_web::{web, App, HttpServer, middleware};
use sqlx::{ PgPool};
use std::{env};
mod UserController {
    pub mod user_routes;
    pub mod user_service;
}
mod models {
    pub mod filter;
}

use UserController::user_routes::*;
use UserController::user_service::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("Using database URL: {}", database_url);
    let pool = PgPool::connect(&database_url).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
            .service(filter_users)
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