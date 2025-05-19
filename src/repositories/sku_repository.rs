// 获取SKU
use sqlx::PgPool;
use crate::models::sku::Sku;

pub struct SkuRepository;

impl SkuRepository {
    pub async fn get_all_sku(pool: &PgPool) -> Result<Vec<Sku>,sqlx::Error> {
        sqlx::query_as::<_,Sku>("SELECT * FROM skus")
            .fetch_all(pool)
            .await
    }
}
