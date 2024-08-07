pub use struct_to_json_db_macro::auto_json_db; 
use std::fs;
use std::io::Write; 
use rand::Rng;
 
pub fn read_string_from_txt(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).unwrap_or_default();
    file_contents
}
pub fn write_string_to_txt(filename: &str, content: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
pub fn unique_id() -> u64 {
    let start = std::time::SystemTime::now();
    let timestamp = start.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64 ;
    let random_number = rand::thread_rng().gen::<u64>();
    timestamp ^ random_number
}

#[macro_export]
macro_rules! auto_json_db_config {
    ($path_str:expr) => {
        use struct_to_json_db::auto_json_db;
        use serde::{Deserialize, Serialize};
        static DB_STRUCT_JSON_PATH:&str = $path_str;
    };
}