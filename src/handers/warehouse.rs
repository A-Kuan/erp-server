use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use crate::services::warehouse_service::WarehouseService;
use crate::app_config::database::DbPool;
use crate::ApiResponse;

#[get("/warehouse")]
pub async fn warehouse(pool: web::Data<DbPool>) -> impl Responder {
    match WarehouseService::get_all_warehouses(pool.get_ref()).await {
        Ok(warehouses) => HttpResponse::Ok().json(ApiResponse{
            code: 200,
            message: "success".to_string(),
            data: warehouses
        }),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}