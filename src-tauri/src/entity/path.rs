use std::env::current_dir;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref program_dir: PathBuf = current_dir().unwrap();
    pub static ref yunzai_dir: PathBuf = program_dir.join(Path::new("Yunzai-bot"));
}
