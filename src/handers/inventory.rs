use actix_web::{get, patch, post, web, HttpResponse, Responder};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json,Query};
use serde_json::json;
use crate::ApiResponse;
use crate::app_config::database::DbPool;
use crate::models::inventory::{Inventory, InventoryBuilder, InventoryQuery, InventoryUpdateBuilder};
use crate::services::inventory_service::InventoryService;
use crate::utils::calamine::{read_excel, ExcelQuery};

/*
    获取所有的库存明细
    # 参数
    -
    # 响应
 */
#[get("/inventories")]
pub async fn inventories(pool: Data<DbPool>) -> impl Responder {
    match InventoryService::get_all_inventories(pool.get_ref()).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::success(inventory)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}

/*
    添加单个库存明细
    # 参数
        bin_id: String
        quantity: i64
        sku: String
        warehouse_id: String
    # 响应
 */
#[post("/inventories")]
pub async fn insert_inventory(pool: Data<DbPool>, request: Json<InventoryBuilder>) -> impl Responder {
    let inventory = InventoryBuilder::new(
        request.warehouse_id.clone(),
        request.bin_id.clone(),
        request.sku.clone(),
        request.quantity.clone(),
    )
        .safety_stock(request.safety_stock)
        .batch_id(request.batch_id.as_deref().unwrap_or(""))
        .build();

    match InventoryService::insert_inventory(pool.get_ref(), inventory).await {
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

    InventoryService::insert_inventories_df(pool.get_ref(), inventories_record).await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body("数据导入成功"))
}

/*
    获取单个库存明细
    # 参数
        sku: String
    # 响应
        Inventory
 */
#[get("/inventory/sku")]
pub async fn get_inventories_by_sku(
    pool: Data<DbPool>,
    query: Query<InventoryQuery>,
) -> impl Responder  {
    // 首先检查是否有提供sku参数
    let sku = match &query.sku {
        Some(sku) => sku,
        None => return HttpResponse::BadRequest().json(json!({
            "error": "sku parameter is required"
        })),
    };
    match InventoryService::get_inventory_by_sku(pool.get_ref(), sku).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::success(inventory)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
#[get("/inventory/id")]
pub async fn get_inventories_by_id(
    pool: Data<DbPool>,
    query: Query<InventoryQuery>,
)-> impl Responder  {
    let id = match &query.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(json!({
            "error": "id parameter is required"
        })),
    };
    match InventoryService::get_inventory_by_id(pool.get_ref(), id).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::success(inventory)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}

/*
    修改明细
 */
#[patch("/inventory/update")]
pub async fn update_inventory(
    pool: Data<DbPool>,
    query: Query<InventoryUpdateBuilder>,
) -> impl Responder {
    match InventoryService::update_inventory(pool.get_ref(), query.into_inner()).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::success(inventory)),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
