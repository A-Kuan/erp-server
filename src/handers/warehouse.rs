use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::services::warehouse_service;
use crate::config::database::DbPool;

pub async fn warehouse(pool: web::Data<DbPool>) -> impl Responder {
    match warehouse_service::get_all_warehouses(pool.get_ref()).await {
        Ok(warehouses) => HttpResponse::Ok().json(warehouses),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}