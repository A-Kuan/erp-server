#[derive(Debug)]
pub enum AppError {
    DataFrameError(String),
    DbError(sqlx::Error),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DataFrameError(e) => write!(f, "DataFrame处理错误: {}", e),
            Self::DbError(e) => write!(f, "数据库错误: {}", e),
            Self::IoError(e) => write!(f, "IO错误: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        Self::DbError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}