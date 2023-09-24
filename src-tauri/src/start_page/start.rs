use std::process::Command;

use encoding::{DecoderTrap, Encoding};
use encoding::all::GBK;
use serde_json::json;

use return_data::ReturnData;

use super::super::entity::{path, return_data};

#[tauri::command]
pub async fn start_yunzai() -> String {
    let a = Command::new("cmd")
        .current_dir(path::yunzai_dir.as_path())
        .args(&["/c", "start", "cmd", "/k", "node app"])
        .output();
    return match a {
        Ok(res) => {
            let out = GBK.decode(&res.stdout, DecoderTrap::Strict).unwrap();
            println!("{out}");
            let data = ReturnData {
                code: 200,
                data: out,
                message: String::from("启动成功"),
            };
            json!(data).to_string()
        }
        Err(err) => {
            json!(ReturnData::<String>::run_failure(err.to_string())).to_string()
        }
    };
}