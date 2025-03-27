use sqlx::{PgPool, postgres::PgConnection, Acquire};
use std::io::Cursor;
use crate::errors::error::AppError;

const COPY_SQL: &str = r#"
    COPY inventory (
        warehouse_id,
        sku,
        bin_id,
        quantity,
        safety_stock
    )
    FROM STDIN
    WITH (
        FORMAT CSV,
        DELIMITER E'\t',
        NULL '\\N'
    )
"#;

pub async fn execute_copy(
    pool: &PgPool,
    data: Cursor<Vec<u8>>,
) -> Result<u64, AppError> {
    let mut conn = pool.acquire().await?;

    // 使用事务保证原子性
    let mut tx = conn.begin().await?;

    // 执行 COPY 命令
    let bytes_copied = PgConnection::copy_in_raw(&mut tx, COPY_SQL)
        .await?
        .send(data)
        .await?;

    tx.commit().await?;

    Ok(bytes_copied)
}