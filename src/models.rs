use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Warehouse {
    warehouse_id: String,
    name: String,
    location: String,
}
