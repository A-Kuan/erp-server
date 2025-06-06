use calamine::{open_workbook, Reader, Xlsx};
use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Deserialize)]
pub struct ExcelQuery {
    /// Excel 文件的路径，例如 "test.xlsx"
    pub(crate) file_path: String,
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