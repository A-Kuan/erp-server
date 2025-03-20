use actix_web::{web, HttpResponse, Responder};
use calamine::{open_workbook, Reader, Xlsx};
use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Deserialize)]
pub struct ExcelQuery {
    /// Excel 文件的路径，例如 "test.xlsx"
    file_path: String,
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
            // 使用 JsonWriter 将 DataFrame 转换为 JSON 字符串
            let mut buffer = Vec::new();
            if JsonWriter::new(&mut buffer)
                .with_json_format(JsonFormat::Json)
                .finish(&mut df)
                .is_err()
            {
                return HttpResponse::InternalServerError().body("将 DataFrame 转换为 JSON 时出错");
            }
            match String::from_utf8(buffer) {
                Ok(json_data) => HttpResponse::Ok().json(json_data),
                Err(e) => HttpResponse::InternalServerError().body(format!("将 JSON 数据转换为字符串时出错：{}", e)),
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // 命令行测试部分
//     let file_path = "test.xlsx"; // 请确保该文件存在于项目根目录
//
//     match read_excel(file_path) {
//         Ok(df) => {
//             println!("成功读取 DataFrame：\n{}", df);
//         },
//         Err(e) => {
//             eprintln!("读取 Excel 文件失败：{}", e);
//         },
//     }
//
//     // 启动 actix-web 服务器，测试 Web 接口
//     println!("启动服务器：访问 http://127.0.0.1:8080/read_excel?file_path=test.xlsx");
//     HttpServer::new(|| {
//         App::new().route("/read_excel", web::get().to(read_excel_handler))
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }
