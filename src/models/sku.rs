use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sku {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub brand: String,
    pub oe: Option<String>,
    pub case_number: i32,
}

#[derive(Deserialize)]
pub struct SkuQuery {
    pub sku: String,
}
#[derive(Debug, Deserialize)]
pub struct SkuBuilder {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub brand: String,
    pub oe: Option<String>,
    pub case_number: i32,
}
impl SkuBuilder {
    pub fn new(
        sku: String,
        name: String,
        unit: String,
        brand: String,
        case_number: i32,
    ) -> Self {
        SkuBuilder {
            sku,
            name,
            description: None,
            unit,
            brand,
            oe: None,
            case_number,
        }

    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn oe(mut self, oe: &str) -> Self {
        self.oe = Some(oe.to_string());
        self
    }

    pub fn build(self) -> Sku {
        let now = Utc::now();
        Sku {
            sku: self.sku,
            name: self.name,
            description: self.description,
            unit: self.unit,
            created_at: now,
            updated_at: now,
            brand: self.brand,
            oe: self.oe,
            case_number: self.case_number,
        }
    }
}
