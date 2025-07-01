use actix_web::{web, get, HttpResponse, Responder, post};
use serde_json::json;
use web::{
    Json,
    Data
};
use crate::app_config::database::DbPool;
use crate::services::sku_service::SkuService;
use crate::ApiResponse;

#[get("/skus")]
pub async fn skus(pool: Data<DbPool>) -> impl Responder {
    match SkuService::get_all_sku(pool.get_ref()).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
// #[get("/get_sku")]
// pub async fn get_sku(pool: Data<DbPool>, query: web::Query<SkuQuery>) -> impl Responder {
//
//     match SkuService::get_sku(pool.get_ref(),&query.sku).await {
//         Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
//         Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
//     }
// }
