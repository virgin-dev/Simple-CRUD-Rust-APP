use crate::system_configure::logger::Logger;
use crate::RoleController::role_routes::{assign_role_to_user, create_role, get_role};
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::time::Duration;

use crate::system_configure::database::Database;
use crate::system_configure::server::AppConfig;
use UserController::user_routes::*;
use UserController::user_service::*;

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
mod system_configure {
    pub mod logger;
    pub mod database;
    pub mod server;
}
fn init_tracing() {
    let logger = Logger::new("app.log", "debug");
    logger.init()
}

async fn init_database_pool(config: &AppConfig) -> PgPool {
    let database = Database::new(&*config.database.host, config.database.port, &*config.database.username, &*config.database.password, &*config.database.database, config.database.max_connections, Duration::new(config.database.acquire_timeout_secs, config.database.acquire_timeout_nanos), Some(Duration::from_secs(config.database.max_lifetime_secs)));

    let pool = database.create_pool().await;
    pool
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    dotenv::dotenv().ok();
    let config = AppConfig::from_env();

    let pool = init_database_pool(&config).await;

    tracing::info!("Loaded configuration: {:?}", &config);

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
            .service(assign_role_to_user)
    })
    .bind((config.server.host, config.server.port))?
        .workers(config.server.workers)
        .backlog(config.server.backlog)
        .max_connections(config.server.max_connections)
        .keep_alive(Duration::from_secs(config.server.keep_alive))
        .client_request_timeout(Duration::from_secs(config.server.client_timeout))
        .client_disconnect_timeout(Duration::from_secs(config.server.client_disconnect_timeout))
        .run()
    .await
}