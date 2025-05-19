use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub max_upload_size: usize,
    pub database_url: String,
    // 可以添加其他配置项，比如数据库连接等
}