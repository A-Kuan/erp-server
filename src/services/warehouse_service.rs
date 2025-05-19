use crate::repositories::warehouse_repository::WarehouseRepository;
use crate::models::warehouse::Warehouse;
use sqlx::PgPool;

pub struct WarehouseService;

impl WarehouseService {
    pub async fn get_all_warehouses(pool: &PgPool) -> Result<Vec<Warehouse>, String> {
        WarehouseRepository::get_all_warehouses(pool)
            .await
            .map_err(|e| e.to_string())
    }
}
