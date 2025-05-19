use actix_web::{web, get, HttpResponse, Responder, post};
use serde_json::json;
use web::{
    Json,
    Data
};
use crate::app_config::database::DbPool;
use crate::services::sku_service::SkuService;
use crate::ApiResponse;
use crate::models::sku::{SkuBuilder, SkuQuery};

#[get("/skus")]
pub async fn skus(pool: Data<DbPool>) -> impl Responder {
    match SkuService::get_all_sku(pool.get_ref()).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
#[get("/get_sku")]
pub async fn get_sku(pool: Data<DbPool>, query: web::Query<SkuQuery>) -> impl Responder {

    match SkuService::get_sku(pool.get_ref(),&query.sku).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
#[post("/sku/create")]
pub async fn create_sku(pool: Data<DbPool>,request: Json<SkuBuilder>) -> impl Responder {
    // 使用构建器模式创建 SKU
    let sku = SkuBuilder::new(
        request.sku.clone(),
        request.name.clone(),
        request.unit.clone(),
        request.brand.clone(),
        request.case_number,
    )
        .description(request.description.as_deref().unwrap_or(""))
        .oe(request.oe.as_deref().unwrap_or(""))
        .build();

    match SkuService::create_sku(pool.get_ref(), sku).await {
        Ok(sku) => HttpResponse::Ok().json(ApiResponse::success(sku)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}