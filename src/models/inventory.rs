use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: i32,
    pub warehouse_id: String,
    pub bin_id: i32,
    pub sku: String,
    pub quantity: i32,
    pub safety_stock: i32,
    pub last_updated: DateTime<Utc>,
}