use actix_web::{web, get, HttpResponse, Responder};
use serde_json::json;
use crate::app_config::database::DbPool;
use crate::services::sku_service::SkuService;
use crate::ApiResponse;

#[get("/skus")]
pub async fn skus(pool: web::Data<DbPool>) -> impl Responder {
    match SkuService::get_all_sku(pool.get_ref()).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}