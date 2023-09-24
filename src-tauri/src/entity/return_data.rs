use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReturnData<T> {
    pub code: i32,
    pub data: T,
    pub message: String,
}

impl<T> ReturnData<T> {
    pub fn failure(code: i32, message: String) -> ReturnData<String> {
        ReturnData {
            code,
            data: String::new(),
            message,
        }
    }

    pub fn fast_success(message: String) -> ReturnData<String> {
        ReturnData {
            code: 200,
            data: String::new(),
            message,
        }
    }

    pub fn run_failure(message: String) -> ReturnData<String> {
        ReturnData {
            code: 500,
            data: String::new(),
            message,
        }
    }
}

