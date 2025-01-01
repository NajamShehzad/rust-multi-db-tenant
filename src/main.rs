mod db;
mod routes;
mod user;
mod todo;
mod utils;

use actix_web::{App, HttpServer, web};
use crate::db::MongoRepo;
use std::sync::Arc;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Initializing MongoDB repository...");
    let mongo_repo = Arc::new(MongoRepo::init().await);
    let data = web::Data::new(mongo_repo);

    info!("Starting Actix Web server on 127.0.0.1:8080...");
    print!("Starting Actix Web server on");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    info!("Server stopped.");
    print!("Server stopped.");
    Ok(())
}
