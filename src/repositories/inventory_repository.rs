//  获取SKU 库存相关
use sqlx::{PgPool, QueryBuilder, Postgres};

use crate::models::inventory::Inventory;
use crate::app_config::TransactionExt;
use sqlx::types::chrono::{ DateTime, Utc };


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

    // 通过sku获取单个库存信息
    pub async fn get_inventory_by_sku(pool: &PgPool, sku: &str) -> Result<Option<Inventory>,sqlx::Error> {
        let inventory = sqlx::query_as!(
            Inventory,
            r#"
            SELECT
                id,
                warehouse_id,
                bin_id,
                sku,
                quantity,
                safety_stock,
                last_updated as "last_updated!: DateTime<Utc>",
                batch_id
            FROM inventories
            WHERE sku = $1
            "#,
            sku
        )
            .fetch_optional(pool)  // 使用 fetch_optional 因为可能找不到记录
            .await?;
        Ok(inventory)
    }
    // 通过id获取单个库存
    pub async fn get_inventory_by_id(pool: &PgPool, id: &str) -> Result<Option<Inventory>,sqlx::Error> {
        let inventory = sqlx::query_as!(
            Inventory,
            r#"
            SELECT
                id,
                warehouse_id,
                bin_id,
                sku,
                quantity,
                safety_stock,
                last_updated as "last_updated!: DateTime<Utc>",
                batch_id
            FROM inventories
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)  // 使用 fetch_optional 因为可能找不到记录
            .await?;
        Ok(inventory)
    }

    // 插入inventory
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

    // 更新inventory
    pub async fn update_inventory(pool: &PgPool, inv: Inventory) -> Result<Inventory,sqlx::Error> {
        pool.with_transaction(|tx| {
            Box::pin(async move {
                sqlx::query_as::<_, Inventory>(
                r#"
                    UPDATE inventories
                    SET
                        warehouse_id = $2,
                        bin_id = $3,
                        sku = $4,
                        quantity = $5,
                        safety_stock = $6,
                        batch_id = $7,
                        last_updated = $8
                    WHERE id = $1
                    RETURNING *
                    "#
                )
                    .bind(&inv.id)
                    .bind(&inv.warehouse_id)
                    .bind(&inv.bin_id)
                    .bind(&inv.sku)
                    .bind(&inv.quantity)
                    .bind(&inv.safety_stock)
                    .bind(&inv.batch_id)
                    .bind(&inv.last_updated)
                    .fetch_one(&mut *tx)
                    .await
            })
        }).await
    }

}
