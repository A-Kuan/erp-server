//  获取SKU 库存相关
use sqlx::PgPool;
use crate::models::inventory::Inventory;
pub async fn get_all_inventory(pool: &PgPool) -> Result<Vec<Inventory>,sqlx::Error> {
    sqlx::query_as::<_,Inventory>("SELECT * FROM inventories")
        .fetch_all(pool)
        .await
}