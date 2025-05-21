use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use polars::prelude::*;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Option<i32>,
    pub warehouse_id: String,
    pub bin_id: i32,
    pub sku: String,
    pub quantity: i32,
    pub safety_stock: Option<i32>,
    pub last_updated: DateTime<Utc>,
    pub batch_id: Option<String>,
}

impl Inventory {
    pub fn dataframe_to_inventory_vec(df: &DataFrame) -> Result<Vec<Inventory>, Box<dyn std::error::Error>> {
        let warehouse_id_series = df.column("warehouse_id")?.str()?; // 已是字符串
        let bin_id_series = df.column("bin_id")?.str()?; // 改为 utf8，再手动转为 i32
        let sku_series = df.column("sku")?.str()?;       // 已是字符串
        let quantity_series = df.column("quantity")?.str()?;
        let safety_stock_series = df.column("safety_stock")?.str()?;
        let batch_id_series = df.column("batch_id")?.str()?;

        let mut inventories = Vec::with_capacity(df.height());

        for i in 0..df.height() {
            let last_updated = Utc::now();

            let bin_id_str = bin_id_series.get(i).ok_or("bin_id is null")?;
            let bin_id: i32 = bin_id_str.parse()?;

            let quantity_str = quantity_series.get(i).ok_or("quantity is null")?;
            let quantity: i32 = quantity_str.parse()?;

            let safety_stock_str = safety_stock_series.get(i).ok_or("safety_stock is null")?;
            let safety_stock: Option<i32> = if safety_stock_str.is_empty() {
                None
            } else {
                Some(safety_stock_str.parse()?)
            };
            let id = Uuid::new_v4(); // 使用 UUID v4 生成一个随机的 UUID

            let batch_id_str = batch_id_series.get(i).ok_or("batch_id is null")?;
            let batch_id: Option<String> = if batch_id_str.is_empty() {
                None
            } else {
                Some(batch_id_str.parse()?)
            };

            let inventory = Inventory {
                id: Some(id.as_u128() as i32
                ),
                warehouse_id: warehouse_id_series.get(i).ok_or("warehouse_id is null")?.to_string(),
                bin_id,
                sku: sku_series.get(i).ok_or("sku is null")?.to_string(),
                quantity,
                safety_stock,
                last_updated,
                batch_id
            };
            inventories.push(inventory);
        }

        Ok(inventories)
    }
}