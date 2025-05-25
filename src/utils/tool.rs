use chrono::{DateTime, Utc};
use uuid::Uuid;
pub fn generate_id() -> String {
    Uuid::now_v7().to_string()
}
pub fn now() -> DateTime<Utc> {
    Utc::now()
}