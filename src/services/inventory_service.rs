use crate::models::inventory::{Inventory, InventoryUpdateBuilder};
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
    pub async fn get_inventory_by_id(pool: &PgPool, id: &str) -> Result<Inventory, String> {
        InventoryRepository::get_inventory_by_id(pool, id)
            .await.
            map_err(|e| e.to_string())
    }

    pub async fn update_inventory(pool: &PgPool,update: InventoryUpdateBuilder) -> Result<Inventory, String> {
        // 1. 获取当前库存记录
        let mut inventory = Self::get_inventory_by_id(pool, &*update.id).await?;
        // 2. 应用更新（只覆盖有值的字段）
        if let Some(warehouse_id) = update.warehouse_id {
            inventory.warehouse_id = warehouse_id;
        }
        if let Some(bin_id) = update.bin_id {
            inventory.bin_id = bin_id;
        }
        if let Some(sku) = update.sku {
            inventory.sku = sku;
        }
        if let Some(quantity) = update.quantity {
            inventory.quantity = quantity;
        }
        if update.safety_stock.is_some() {
            inventory.safety_stock = update.safety_stock;
        }
        if update.batch_id.is_some() {
            inventory.batch_id = update.batch_id;
        }
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
