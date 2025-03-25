use crate::repositories::sku_repository;
use crate::models::sku::Sku;
use sqlx::PgPool;

pub async fn get_all_sku(pool: &PgPool) -> Result<Vec<Sku>, String> {
    sku_repository::get_all_sku(pool)
        .await
        .map_err(|e| e.to_string())
}