use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u32, message: String, data: T) -> ApiResponse<T> {
        ApiResponse { code, message, data }
    }

    pub fn success(data: T) -> ApiResponse<T> {
        ApiResponse { code: 200, message: "success".to_string(), data }
    }
}