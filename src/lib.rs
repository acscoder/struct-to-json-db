pub use struct_to_json_db_macro::auto_json_db; 
use std::fs;
use std::io::Write; 
use rand::Rng;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
pub use paste::paste;

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
pub fn write_string_to_txt_encript(filename: &str, content: String,encript:&str) {
    write_string_to_txt(filename, string_to_crypt(content,encript));
}
pub fn read_string_from_txt_encript(filename: &str,encript:&str) -> String {
    let file_contents = read_string_from_txt(filename);
    crypt_to_string(file_contents,encript)
}
fn string_to_crypt(s: String,encript:&str) -> String {
    let mc = new_magic_crypt!(encript, 256);
    mc.encrypt_str_to_base64(s)
}
fn crypt_to_string(s: String,encript:&str) -> String {
    let mc = new_magic_crypt!(encript, 256);
    match mc.decrypt_base64_to_string(&s) {
        Ok(r) => r,
        Err(_) => s,
    }
}

pub fn unique_id() -> (u64,u64) {
    let start = std::time::SystemTime::now();
    let timestamp = start.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64 ;
    let random_number = rand::thread_rng().gen::<u64>();
    (random_number,timestamp)
}

#[macro_export]
macro_rules! json_db_get_by {
    ($first:ident, $second:ident:$second_type:expr) => {
        struct_to_json_db::paste! {
            |all_data:&HashMap<u64, [<$first>]>,byval:[<$second_type>]|->Vec<(u64,[<$first>])>{
                all_data.iter().filter_map(|(id, obj)| {
                    if obj.[<$second>] == byval {
                        Some((id.clone(), obj.clone()))
                    } else {
                        None
                    }
                    }).collect()      
            }
        }
    };
}

#[macro_export]
macro_rules! auto_json_db_config {
    ($path_str:expr) => {
        use struct_to_json_db::*;
        use serde::{Deserialize, Serialize};
        static DB_STRUCT_JSON_PATH:&str = $path_str;
    };
}
#[macro_export]
macro_rules! auto_json_db_single{
    ($struct:ident) => {
        struct_to_json_db::paste! {
            impl [<$struct>] {
                pub fn save(&self) {
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(self).unwrap();
                    write_string_to_txt(&file_path, json_data);
                }
                pub fn load()->Self{
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let file_contents = read_string_from_txt(&file_path);
                    if file_contents.is_empty() {
                        [<$struct>]::default()

                    }else{
                        let r = serde_json::from_str(&file_contents).ok();
                        r.unwrap_or_default()
                    }
                    
                }
            }
        }
    };
    ($struct:ident, $pass:literal) => {
        struct_to_json_db::paste! {
           
            impl [<$struct>] {
                pub fn save(&self) {
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(self).unwrap();
                    write_string_to_txt_encript(&file_path, json_data,$pass);
                }
                pub fn load()->Self{
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let file_contents = read_string_from_txt_encript(&file_path,$pass);
                    if file_contents.is_empty() {
                        [<$struct>]::default()
                    }else{
                        let r = serde_json::from_str(&file_contents).ok();
                        r.unwrap_or_default()
                    }
                    
                }
            }        
            
        }
    };
}
#[macro_export]
macro_rules! auto_json_db_one_file{
    ($struct:ident) => {
        struct_to_json_db::paste! {
            impl [<$struct>] {
                pub fn save(&self) {
                    let mut data = Self::load();
                    let is_available = data.iter().any(|i| i == self);
                    if !is_available {
                        data.push(self.clone());
                        let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                        let json_data = serde_json::to_string(&data).unwrap();
                        write_string_to_txt(&file_path, json_data);
                    }
                }
                pub fn remove(&self){
                    let mut data = Self::load();
                    let is_available = data.iter().any(|i| i == self);
                    if is_available {
                        data.retain(|i| i != self);
                        let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                        let json_data = serde_json::to_string(&data).unwrap();
                        write_string_to_txt(&file_path, json_data);
                    }
                }
                pub fn save_all(data:&Vec<Self>){
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(data).unwrap();
                    write_string_to_txt(&file_path, json_data);
                }
                pub fn load()->Vec<Self>{
                    let file_path = DB_STRUCT_JSON_PATH.to_string() + "/" + stringify!($struct) + ".json";
                    let file_contents = read_string_from_txt(&file_path);
                    if file_contents.is_empty() {
                        vec![]
                    }else{
                        let r = serde_json::from_str(&file_contents).ok();
                        r.unwrap_or(vec![])
                    }
                    
                }
            } 
        } 
    }
}

#[macro_export]
macro_rules! json_db_relation {
    ($first:ident=$field:ident, $second:ident) => {
        struct_to_json_db::paste! {
            impl [<$first>] {
                pub fn [<get_ $field>](&self)->Vec<[<$second>]>{
                    let data = [<$second>]::get_by_ids(&self.[<$field>]);
                    data
                }
                pub fn [<set_ $field>](&mut self,v:&Vec<[<$second>]>){
                    self.[<$field>] = v.iter().map(|item| item.idx).collect();
                    self.save();
                } 
            }           
        }
    };
    ($first:ident=$field:ident, $second:ident,"1:1") => {
        struct_to_json_db::paste! {
            impl [<$first>] {
                pub fn [<get_ $field>](&self)->Option<[<$second>]>{
                    let data = [<$second>]::get_by_id(self.[<$field>]);
                    data
                }
                pub fn [<set_ $field>](&mut self,v:&[<$second>]){
                    self.[<$field>] = v.idx;
                    self.save();
                } 
            }           
        }
    };
    
}
 