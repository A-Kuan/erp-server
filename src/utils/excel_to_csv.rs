use polars::prelude::*;
use std::io::Cursor;
use crate::errors::error::AppError;

pub fn dataframe_to_csv(
    df: &DataFrame,
) -> Result<Cursor<Vec<u8>>, AppError> {
    let mut buffer = Vec::new();

    // 验证必要列存在
    let required_cols = ["warehouse_id", "sku", "bin_id", "quantity"];
    for col in required_cols {
        if !df.get_column_names().contains(&col) {
            return Err(AppError::DataFrameError(format!("缺少必要列: {}", col)));
        }
    }

    // 构建 CSV Writer
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(&mut buffer);

    // 获取各列数据
    let warehouse_ids = df.column("warehouse_id")?.i32()?;
    let skus = df.column("sku")?.utf8()?;
    let bin_ids = df.column("bin_id")?.i32()?;
    let quantities = df.column("quantity")?.i32()?;
    let safety_stocks = df.column("safety_stock")
        .map(|c| c.i32()?.as_optional())
        .unwrap_or_else(|_| Some(vec![None; df.height()]));

    // 写入数据
    for idx in 0..df.height() {
        let safety_stock = match safety_stocks.as_ref().and_then(|s| s.get(idx)) {
            Some(Some(v)) => v.to_string(),
            _ => "\\N".to_string(),
        };

        writer.write_record(&[
            warehouse_ids.get(idx).unwrap_or_default().to_string(),
            skus.get(idx).unwrap_or_default().to_string(),
            bin_ids.get(idx).unwrap_or_default().to_string(),
            quantities.get(idx).unwrap_or_default().to_string(),
            safety_stock,
        ])?;
    }

    writer.flush()?;
    Ok(Cursor::new(buffer))
}