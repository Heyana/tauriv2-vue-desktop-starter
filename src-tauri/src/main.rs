// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

mod utils;
#[tauri::command]
fn test_magick(json: String) -> Result<Vec<u8>, String> {
    println!("test_magick called with json: {}", json);
    // 解析 JSON 字符串
    let data: MyData = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let res = utils::image_magick::image_magick_convert_tokio(&data).unwrap();

    println!("ImageMagick result: {}", res.len());
    // 打印解析后的数据
    Ok(res)
}

// 定义一个数据结构，用于解析 JSON 数据
#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    files: Vec<String>,
    format: String,
    quality: u8,
}

fn main() {
    let mut dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_magick])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
