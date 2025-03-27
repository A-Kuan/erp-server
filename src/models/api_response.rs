use serde::{Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: u16, message: &str, data: Option<Value>) -> Self {
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }
}