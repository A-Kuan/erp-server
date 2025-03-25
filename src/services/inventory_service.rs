use crate::repositories::inventory_repository;
use crate::models::inventory::Inventory;
use sqlx::PgPool;

pub async fn get_all_inventories(pool: &PgPool) -> Result<Vec<Inventory>, String> {
    inventory_repository::get_all_inventory(pool)
        .await
        .map_err(|e| e.to_string())
}