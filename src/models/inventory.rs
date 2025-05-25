use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use polars::prelude::*;
use sqlx::FromRow;
use crate::utils::tool::{ generate_id};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: String,
    pub warehouse_id: String,
    pub bin_id: String,
    pub sku: String,
    pub quantity: i32,
    pub safety_stock: Option<i32>,
    pub last_updated: DateTime<Utc>,
    pub batch_id: Option<String>,
}
#[derive(Deserialize)]
pub struct InventoryQuery {
    pub sku: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryUpdateBuilder {
    pub id: String,
    pub warehouse_id: Option<String>,
    pub sku: Option<String>,
    pub bin_id: Option<String>,
    pub quantity: Option<i32>,
    pub safety_stock: Option<i32>,
    pub batch_id: Option<String>,

}
#[derive(Debug, Deserialize)]
pub struct InventoryBuilder {
    pub warehouse_id: String,
    pub bin_id: String,
    pub sku: String,
    pub quantity: i32,
    pub safety_stock: Option<i32>,
    pub batch_id: Option<String>,
}

impl InventoryBuilder {
    pub fn new(
        warehouse_id: String,
        bin_id: String,
        sku: String,
        quantity: i32,
    ) -> Self {
        InventoryBuilder {
            warehouse_id,
            bin_id,
            sku,
            quantity,
            safety_stock: None,
            batch_id: None,
        }
    }

    pub fn safety_stock(mut self, safety_stock: Option<i32>) -> Self {
        self.safety_stock = safety_stock;
        self
    }

    pub fn batch_id(mut self, batch_id: &str) -> Self {
        self.batch_id = Some(batch_id.to_string());
        self
    }

    pub fn build(self) -> Inventory {
        let id = generate_id();
        let now = Utc::now();

        Inventory {
            id,
            warehouse_id: self.warehouse_id,
            bin_id: self.bin_id,
            sku: self.sku,
            quantity: self.quantity,
            safety_stock: self.safety_stock,
            batch_id: self.batch_id,
            last_updated: now
        }
    }
}

impl InventoryUpdateBuilder{
    pub fn new (
        id: String,
    ) -> Self {
        InventoryUpdateBuilder {
            id,
            warehouse_id: None,
            sku: None,
            bin_id: None,
            quantity: None,
            safety_stock: None,
            batch_id: None,
        }
    }
    pub fn warehouse_id(mut self, warehouse_id: &str) -> Self{
        self.warehouse_id = Some(warehouse_id.to_string());
        self
    }
    pub fn sku(mut self, sku: &str) -> Self {
        self.sku = Some(sku.to_string());
        self
    }
    pub fn bin_id(mut self, bin_id: &str) -> Self {
        self.bin_id = Some(bin_id.to_string());
        self
    }
    pub fn quantity(mut self, quantity: i32) -> Self {
        self.quantity = Some(quantity);
        self
    }
    pub fn safety_stock(mut self, safety_stock: Option<i32>) -> Self {
        self.safety_stock = safety_stock;
        self
    }
    pub fn batch_id(mut self, batch_id: &str) -> Self {
        self.batch_id = Some(batch_id.parse().unwrap());
        self
    }

    pub fn build(self) -> InventoryUpdateBuilder {
        InventoryUpdateBuilder {
            id: self.id,
            warehouse_id: self.warehouse_id,
            bin_id: self.bin_id,
            sku: self.sku,
            quantity: self.quantity,
            safety_stock: self.safety_stock,
            batch_id: self.batch_id,
        }
    }
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
            let bin_id = bin_id_str.parse()?;

            let quantity_str = quantity_series.get(i).ok_or("quantity is null")?;
            let quantity: i32 = quantity_str.parse()?;

            let safety_stock_str = safety_stock_series.get(i).ok_or("safety_stock is null")?;
            let safety_stock: Option<i32> = if safety_stock_str.is_empty() {
                None
            } else {
                Some(safety_stock_str.parse()?)
            };

            let batch_id_str = batch_id_series.get(i).ok_or("batch_id is null")?;
            let batch_id: Option<String> = if batch_id_str.is_empty() {
                None
            } else {
                Some(batch_id_str.parse()?)
            };

            let inventory = Inventory {
                id: generate_id(),
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