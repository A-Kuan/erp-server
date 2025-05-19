use sqlx::PgPool;
use crate::models::warehouse::Warehouse;

pub struct WarehouseRepository;

impl WarehouseRepository {
    pub async fn get_all_warehouses(pool: &PgPool) -> Result<Vec<Warehouse>, sqlx::Error> {
        sqlx::query_as::<_, Warehouse>("SELECT * FROM warehouses")
            .fetch_all(pool)
            .await
    }
}

