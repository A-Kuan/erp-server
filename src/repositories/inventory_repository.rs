//  获取SKU 库存相关
use sqlx::{PgPool, QueryBuilder, Postgres};

use crate::models::inventory::Inventory;
use crate::app_config::TransactionExt;
pub struct InventoryRepository;

impl InventoryRepository {
    pub async fn bulk_upsert_inventories(
        pool: &PgPool,
        inventories: &[Inventory],
    ) -> Result<(), sqlx::Error> {
        if inventories.is_empty() {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO inventories (id, warehouse_id, bin_id, sku, quantity, safety_stock, last_updated, batch_id)");

        query_builder.push_values(inventories, |mut b, inv| {
            b.push_bind(inv.id.clone())
                .push_bind(&inv.warehouse_id)
                .push_bind(&inv.bin_id)
                .push_bind(&inv.sku)
                .push_bind(inv.quantity)
                .push_bind(inv.safety_stock)
                .push_bind(inv.last_updated)
                .push_bind(&inv.batch_id);
        });

        // ON CONFLICT 更新 quantity 和 last_updated
        query_builder.push(
            " ON CONFLICT (id) DO UPDATE SET quantity = EXCLUDED.quantity, last_updated = EXCLUDED.last_updated",
        );

        let query = query_builder.build();
        query.execute(pool).await?;

        Ok(())
    }

    // 获取所有库存
    pub async fn get_all_inventory(pool: &PgPool) -> Result<Vec<Inventory>,sqlx::Error> {
        sqlx::query_as::<_,Inventory>("SELECT * FROM inventories")
            .fetch_all(pool)
            .await
    }

    // 插入sku
    pub async fn insert_inventory(pool: &PgPool, inv: Inventory) -> Result<Inventory,sqlx::Error> {
        pool.with_transaction(|tx| {
            Box::pin(async move {
                sqlx::query_as::<_, Inventory>(
                r#"
                    INSERT INTO inventories
                        (id, warehouse_id, bin_id, sku, quantity, safety_stock, last_updated, batch_id)
                    VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING *
                    "#
                )
                    .bind(&inv.id)
                    .bind(&inv.warehouse_id)
                    .bind(&inv.bin_id)
                    .bind(&inv.sku)
                    .bind(&inv.quantity)
                    .bind(&inv.safety_stock)
                    .bind(&inv.last_updated)
                    .bind(&inv.batch_id)
                    .fetch_one(&mut *tx)
                    .await
            })
        }).await
    }


}
