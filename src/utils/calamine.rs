use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use calamine::{open_workbook, Reader, Xlsx};
use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use serde_json::{Value, json};
use crate::{ApiResponse, ErrorDetail};
use sqlx::{PgPool, postgres::PgCopyIn};
use polars::prelude::*;
use std::io::Cursor;
use polars::prelude::*;

#[derive(Deserialize)]
pub struct ExcelQuery {
    /// Excel 文件的路径，例如 "test.xlsx"
    file_path: String,
}

pub struct InventoryRecord {
    pub warehouse_id: i32,
    pub sku: String,
    pub bin_id: i32,
    pub quantity: i32,
    pub safety_stock: Option<i32>,
}

/// 读取 Excel 文件并转换为 DataFrame
pub fn read_excel<P: AsRef<Path>>(file_path: P) -> Result<DataFrame, Box<dyn Error>> {
    // 打开 Excel 文件
    let mut workbook: Xlsx<_> = open_workbook(file_path)?;
    // 获取第一个工作表名称
    let binding = workbook.sheet_names();
    let sheet_name = binding.get(0)
        .ok_or("Excel 文件没有工作表")?;

    // 直接尝试获取指定工作表的范围
    let range = workbook.worksheet_range(sheet_name)?;

    // 将每行数据转换为 Vec<String>
    let mut rows: Vec<Vec<String>> = range.rows()
        .map(|r| r.iter().map(|v| v.to_string()).collect())
        .collect();

    // 获取表头（假设第一行为表头）
    let headers = if !rows.is_empty() {
        rows.remove(0)
    } else {
        return Err("Excel 文件没有内容".into());
    };
    // 处理重复或空的列名
    let mut unique_headers = Vec::with_capacity(headers.len());
    let mut seen = std::collections::HashSet::new();

    for (i, header) in headers.into_iter().enumerate() {
        let mut new_header = if header.trim().is_empty() {
            format!("column_{}", i + 1)
        } else {
            header
        };
        while !seen.insert(new_header.clone()) {
            new_header.push('_');
        }
        unique_headers.push(new_header);
    }

    // 构建每一列数据
    let series_vec: Vec<Series> = unique_headers.iter().enumerate().map(|(i, header)| {
        let column_data: Vec<String> = rows.iter()
            .map(|row| row.get(i).cloned().unwrap_or_default())
            .collect();
        Series::new(PlSmallStr::from(header), column_data)
    }).collect();

    // 构建 DataFrame
    let columns: Vec<Column> = series_vec.into_iter().map(|s| s.into()).collect();
    let df = DataFrame::new(columns)?;

    Ok(df)
}

/// actix-web 的处理函数，接收查询参数并返回读取结果
/// actix-web 的处理函数，接收查询参数并返回读取结果
pub async fn read_excel_handler(query: web::Query<ExcelQuery>) -> impl Responder {
    match read_excel(&query.file_path) {
        Ok(mut df) => {
            let mut buffer = Vec::new();

            // 将 DataFrame 转换为 JSON
            if JsonWriter::new(&mut buffer)
                .with_json_format(JsonFormat::Json)
                .finish(&mut df)
                .is_err()
            {
                return HttpResponse::InternalServerError().json(ApiResponse {
                    code: 500,
                    message: "DataFrame 转 JSON 失败".to_string(),
                    data: Value::Null,
                });
            }

            // 解析 JSON 字符串为 serde_json::Value
            match serde_json::from_slice::<Value>(&buffer) {
                Ok(json_value) => HttpResponse::Ok().json(ApiResponse {
                    code: 200,
                    message: "文件解析成功".to_string(),
                    data: json_value,  // 直接使用解析后的 JSON
                }),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                    code: 500,
                    message: "JSON 解析失败".to_string(),
                    data: json!({"error": e.to_string()}),
                }),
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            code: 500,
            message: "读取 Excel 失败".to_string(),
            data: json!({"error": e.to_string()}),
        }),
    }
}

pub fn df_to_copy_csv(df: &DataFrame) -> Result<Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(&mut buffer);

    // 确保列顺序与数据库表结构一致（排除自增字段）
    let warehouse_ids = df.column("warehouse_id")?.i32()?;
    let skus = df.column("sku")?.utf8()?;
    let bin_ids = df.column("bin_id")?.i32()?;
    let quantities = df.column("quantity")?.i32()?;
    let safety_stocks = df.column("safety_stock")?.i32()?.as_optional();

    for idx in 0..df.height() {
        let safety_stock = match safety_stocks.get(idx) {
            Some(Some(v)) => v.to_string(),
            Some(None) | None => "\\N".to_string(), // 使用 PostgresSQL 的 NULL 表示
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

pub async fn fast_copy(
    pool: &PgPool,
    data: Cursor<Vec<u8>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;

    // 明确的列指定（排除自增字段）
    let copy_sql = r#"
        COPY inventory (warehouse_id, sku, bin_id, quantity, safety_stock)
        FROM STDIN WITH (FORMAT CSV, DELIMITER E'\t', NULL '\N')
    "#;

    let copy_in = PgCopyIn::new(copy_sql)
        .await?
        .raw(true);

    let mut stream = copy_in.stream(&mut conn);
    let bytes_copied = stream.send(data).await?;
    stream.finish().await?;

    Ok(bytes_copied)
}

pub async fn import_inventory(
    query: web::Query<ExcelQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // 读取 Excel 文件
    let df = match read_excel(&query.file_path) {
        Ok(df) => df,
        Err(e) => return error_response(500, "Excel 读取失败", &e),
    };

    // 校验必要列存在
    if let Err(e) = validate_columns(&df, &["warehouse_id", "sku", "bin_id", "quantity", "safety_stock"]) {
        return error_response(400, "Excel 列校验失败", &e);
    }

    // 转换为 COPY 格式数据
    let cursor = match df_to_copy_csv(&df) {
        Ok(c) => c,
        Err(e) => return error_response(500, "CSV 转换失败", &e),
    };

    // 执行 COPY 命令
    match fast_copy(&pool, cursor).await {
        Ok(bytes) => HttpResponse::Ok().json(ApiResponse {
            code: 200,
            message: format!("成功导入数据 ({} bytes)", bytes),
            data: serde_json::json!(null),
        }),
        Err(e) => error_response(500, "数据库写入失败", &e),
    }
}

fn validate_columns(df: &DataFrame, required: &[&str]) -> Result<(), String> {
    let existing = df.get_column_names();
    for &col in required {
        if !existing.contains(&col) {
            return Err(format!("缺少必要列: {}", col));
        }
    }
    Ok(())
}

pub fn error_response(
    status_code: u16,
    message: &str,
    error: Option<&dyn std::error::Error>,
) -> HttpResponse {
    // 转换状态码
    let status = StatusCode::from_u16(status_code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

    // 构建错误详情
    let error_detail = ErrorDetail {
        error: error.map(|e| e.to_string()),
        trace_id: Some(uuid::Uuid::new_v4().to_string()), // 生成唯一追踪ID
    };

    // 记录日志（根据实际日志库调整）
    log::error!(
        "Error occurred: {} - Trace ID: {}",
        message,
        error_detail.trace_id.as_ref().unwrap()
    );

    HttpResponse::build(status).json(ApiResponse {
        code: status_code,
        message: message.to_owned(),
        data: Some(error_detail),
    })
}