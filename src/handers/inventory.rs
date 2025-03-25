use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::ApiResponse;
use crate::config::database::DbPool;
use crate::services::{inventory_service};

pub async fn inventories(pool: web::Data<DbPool>) -> impl Responder {
    match inventory_service::get_all_inventories(pool.get_ref()).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse{
            code: 200,
            message: "success".to_string(),
            data: inventory
        }),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}