// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::json;

use entity::{return_data, student};

mod utils;
mod start_page;

mod entity;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_message,start_page::start::start_yunzai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    let num = 10;
    let result = utils::cal_utils::plus(num);
    println!("Result: {}", result);
}


#[tauri::command]
fn get_message() -> String {
    let data = return_data::ReturnData {
        code: 200,
        data: student::Student {
            age: 18,
            name: String::from("熊宇正"),

        },
        message: String::new(),
    };
    return json!(data).to_string()
}


