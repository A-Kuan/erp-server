use crate::models::inventory::Inventory;
use sqlx::PgPool;
use crate::repositories::inventory_repository::InventoryRepository;

pub struct InventoryService;


impl InventoryService {
    pub async fn insert_inventories_df(pool: &PgPool, inventories: Vec<Inventory>) -> Result<(), sqlx::Error> {
        InventoryRepository::bulk_upsert_inventories(pool, &inventories).await
    }

    pub async fn insert_inventory(pool: &PgPool, inventory: Inventory) -> Result<Inventory, String> {
        InventoryRepository::insert_inventory(pool,inventory)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_inventory_by_sku(pool: &PgPool, sku: &str) -> Result<Option<Inventory>, String> {
        InventoryRepository::get_inventory_by_sku(pool,sku)
            .await.
            map_err(|e| e.to_string())
    }
    pub async fn get_inventory_by_id(pool: &PgPool, id: &str) -> Result<Option<Inventory>, String> {
        InventoryRepository::get_inventory_by_id(pool, id)
            .await.
            map_err(|e| e.to_string())
    }

    pub async fn update_inventory(pool: &PgPool, inventory: Inventory) -> Result<Inventory, String> {
        InventoryRepository::update_inventory(pool,inventory)
            .await
            .map_err(|e| e.to_string())
    }
    pub async fn get_all_inventories(pool: &PgPool) -> Result<Vec<Inventory>, String> {
        InventoryRepository::get_all_inventory(pool)
            .await
            .map_err(|e| e.to_string())
    }
}
