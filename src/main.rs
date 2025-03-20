use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use crate::handers::warehouse;
use crate::config::database::create_pool;
use crate::utils::calamine;

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
    // 命令行测试部分
    // let file_path = "static/test.xlsx";
    //
    // match calamine::read_excel(file_path) {
    //     Ok(df) => {
    //         println!("成功读取 DataFrame：\n{}", df);
    //     },
    //     Err(e) => {
    //         eprintln!("读取 Excel 文件失败：{}", e);
    //     },
    // }
    println!("启动服务器：访问 http://127.0.0.1:8080/read_excel?file_path=static/test.xlsx");

    HttpServer::new(move || {
        App::new()
            // 配置上传限制（最大1GB）
            .app_data(web::PayloadConfig::new(1024 * 1024 * 1024))
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            // .route("/read_excel", web::get().to(crate::calamine::read_excel_handler))
            .route("/warehouse", web::get().to(warehouse::warehouse))
            .route("/read_excel", web::get().to(calamine::read_excel_handler))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}