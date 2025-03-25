use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use crate::handers::{ warehouse,sku,inventory };
use crate::config::database::create_pool;
use crate::utils::calamine;

use crate::models::api_response;
pub use api_response::ApiResponse;

mod models;
mod handers;
mod services;
mod repositories;
mod utils;
mod config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url).await?;
    let current_dir = env::current_dir().unwrap();
    println!("Current working directory: {:?}", current_dir);
    // println!("启动服务器：访问 http://127.0.0.1:8080/read_excel?file_path=static/test.xlsx");

    HttpServer::new(move || {
        App::new()
            // 配置上传限制（最大1GB）
            .app_data(web::PayloadConfig::new(1024 * 1024 * 1024))
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            // .route("/read_excel", web::get().to(crate::calamine::read_excel_handler))
            .route("/warehouse", web::get().to(warehouse::warehouse))
            .route("/read_excel", web::get().to(calamine::read_excel_handler))
            .route("/get_all_sku", web::get().to(sku::skus))
            .route("/inventories",web::get().to(inventory::inventories))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}