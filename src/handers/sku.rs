use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::config::database::DbPool;
use crate::services::{sku_service};
use crate::ApiResponse;

pub async fn skus(pool: web::Data<DbPool>) -> impl Responder {
    match sku_service::get_all_sku(pool.get_ref()).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}