use crate::models::inventory::Inventory;
use sqlx::PgPool;
use crate::repositories::inventory_repository::InventoryRepository;

pub struct InventoryService;


impl InventoryService {
    pub async fn insert_inventories(pool: &PgPool, inventories: Vec<Inventory>) -> Result<(), sqlx::Error> {
        InventoryRepository::bulk_upsert_inventories(pool, &inventories).await
    }

    pub async fn get_all_inventories(pool: &PgPool) -> Result<Vec<Inventory>, String> {
        InventoryRepository::get_all_inventory(pool)
            .await
            .map_err(|e| e.to_string())
    }
}
