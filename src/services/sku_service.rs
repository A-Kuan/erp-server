use crate::repositories::sku_repository::SkuRepository;
use crate::models::sku::Sku;
use sqlx::PgPool;

pub struct SkuService;
impl SkuService {
    pub async fn get_all_sku(pool: &PgPool) -> Result<Vec<Sku>, String> {
        SkuRepository::get_all_sku(pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_sku(pool: &PgPool, sku: &str) -> Result<Option<Sku>, String> {
        SkuRepository::get_sku(pool, sku)
            .await
            .map_err(|e| e.to_string())
    }
}
