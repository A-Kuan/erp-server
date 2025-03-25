use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Inventory {
    pub id: i32,
    pub warehouse_id: i32,
    pub bin_id: i32,
    pub sku: String,
    pub quantity: i32,
    pub safety_stock: i32,
    pub last_updated: DateTime<Utc>,
}