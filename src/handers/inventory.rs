use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::error::ErrorInternalServerError;
use serde_json::json;
use crate::ApiResponse;
use crate::app_config::database::DbPool;
use crate::models::inventory::Inventory;
use crate::services::inventory_service::InventoryService;
use crate::utils::calamine::{read_excel, ExcelQuery};

/*
    获取所有的库存明细
    # 参数
    -
    # 响应


 */
#[get("/inventories")]
pub async fn inventories(pool: web::Data<DbPool>) -> impl Responder {
    match InventoryService::get_all_inventories(pool.get_ref()).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::success(inventory)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}

// 导入库存明细
#[post("/insert_excel")]
pub async fn import_excel_to_db(query: web::Query<ExcelQuery>, pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let df = read_excel(&query.file_path)
        .map_err(ErrorInternalServerError)?; // 转换错误

    let inventories_record = Inventory::dataframe_to_inventory_vec(&df)
        .map_err(ErrorInternalServerError)?;

    InventoryService::insert_inventories(pool.get_ref(), inventories_record).await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body("数据导入成功"))

}