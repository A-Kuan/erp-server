mod models;

use models::Warehouse;
use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;

use std::env;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/location")]
async fn get_location(pool: web::Data<MySqlPool>) -> impl Responder {
    // 使用 SQL 查询获取用户数据
    let result = sqlx::query_as::<_, Warehouse>("SELECT * FROM warehouse")
        .fetch_all(pool.get_ref())
        .await;

    // 错误处理与返回数据
    match result {
        Ok(locations) => HttpResponse::Ok().json(locations), // 成功返回 JSON 格式的用户数据
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching locations: {}", e)),
    }
}
#[post("/insertLocation")]
async fn insert_location(pool: web::Data<MySqlPool>) -> impl Responder {
    // 准备要插入的数据
    let warehouse_id = "test";
    let name = "test-name";
    let location = "location";

    // 执行数据库插入操作
    let result = sqlx::query!(
        r#"
        INSERT INTO warehouse (warehouse_id, name, location)
        VALUES (?, ?, ?)
        "#,
        warehouse_id,
        name,
        location
    )
        .execute(pool.get_ref())
        .await;

    // 检查插入结果并返回响应
    match result {
        Ok(_) => HttpResponse::Ok().json("Location inserted successfully"),
        Err(err) => {
            eprintln!("Failed to insert location: {}", err);
            HttpResponse::InternalServerError().body("Failed to insert location")
        }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok(); // 读取 .env 文件

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(get_location)
            .service(insert_location)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}