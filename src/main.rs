use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use config::{Config, Environment};
use crate::app_config::database::{
    create_pool
};
use crate::app_config::config::{
    AppConfig
};
use crate::app_config::run::{
    configure_services
};

use crate::models::api_response;
pub use api_response::ApiResponse;

mod models;
mod handers;
mod services;
mod repositories;
mod utils;
mod app_config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    dotenv::dotenv().ok(); // 加载 .env 文件

    Config::builder()
        // 首先从 app_config/default.toml 加载默认值（可选）
        // .add_source(File::with_name("app_config/default"))
        // 然后从 .env 文件覆盖
        .add_source(Environment::default())
        .build()?
        .try_deserialize()
}
#[actix_web::main]
async fn main() -> Result<()> {
    // 加载配置
    let config = load_config().expect("Failed to load configuration");

    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(config.max_upload_size))
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_services)
    })
    .bind((config.server_host, config.server_port))?
    .run()
    .await?;

    Ok(())
}