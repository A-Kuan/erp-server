// 获取SKU
use sqlx::PgPool;
use crate::models::sku::Sku;
use crate::app_config::TransactionExt;

pub struct SkuRepository;

impl SkuRepository {
    // 获取所有sku
    pub async fn get_all_sku(pool: &PgPool) -> Result<Vec<Sku>,sqlx::Error> {
        sqlx::query_as::<_,Sku>("SELECT * FROM base_sku")
            .fetch_all(pool)
            .await
    }

    // 新建sku
    // pub async fn create_sku(pool: &PgPool,sku: Sku) -> Result<Sku,sqlx::Error> {
    //     pool.with_transaction(|tx| {
    //         Box::pin(async move {
    //             sqlx::query_as::<_,Sku>(
    //             r#"
    //                 INSERT INTO skus
    //                     (sku, name, description, unit, created_at, updated_at, brand, oe, case_number)
    //                 VALUES
    //                     ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    //                 RETURNING *
    //                 "#
    //             )
    //                 .bind(&sku.sku)
    //                 .bind(&sku.name)
    //                 .bind(&sku.description)
    //                 .bind(&sku.unit)
    //                 .bind(&sku.created_at)
    //                 .bind(&sku.updated_at)
    //                 .bind(&sku.brand)
    //                 .bind(&sku.oe)
    //                 .bind(&sku.case_number)
    //                 .fetch_one(&mut *tx)
    //                 .await
    //         })
    //     }).await
    // }

    // 查询单个sku
    pub async fn get_sku(pool: &PgPool, sku: &str) -> Result<Option<Sku>,sqlx::Error> {
        let sku_record = sqlx::query_as::<_, Sku>("SELECT * FROM skus WHERE sku = $1")
            .bind(sku)
            .fetch_optional(pool)
            .await?;
        Ok(sku_record)
    }
}
