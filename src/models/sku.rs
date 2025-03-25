use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sku {
    pub sku: String,
    pub name: String,
    pub description: String,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub brand: String,
    pub oe: Option<String>,
    pub case_number: i32,
}