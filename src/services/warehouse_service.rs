use crate::repositories::warehouse_repository;
use crate::models::warehouse::Warehouse;
use sqlx::PgPool;

pub async fn get_all_warehouses(pool: &PgPool) -> Result<Vec<Warehouse>, String> {
    warehouse_repository::get_all_warehouses(pool)
        .await
        .map_err(|e| e.to_string())
}