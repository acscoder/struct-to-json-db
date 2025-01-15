pub use struct_to_json_db_macro::auto_json_db; 
use std::fs;
use std::path::Path;
use std::io::Write; 
use rand::Rng;
use magic_crypt::{ new_magic_crypt, MagicCryptTrait};
pub use paste::paste;
use lazy_static::lazy_static;
use std::sync::Mutex;

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
pub fn make_folder_if_not_exist(path:&str){
    let path = Path::new(path);
    if !path.exists() {
        let _ = fs::create_dir_all(path) ;
    }
}
 
pub fn remove_file_by_path(path: &str) {
    let path = Path::new(path);
    if path.exists() {
        let _ = fs::remove_file(path);
    }  
}
pub fn remove_all_files_by_path(path: &str)  {
    // Convert the input string to a Path
    let path = Path::new(path);
    if path.exists() {
        // Remove the directory and all its contents (files and subdirectories)
        let _ = fs::remove_dir_all(path);
    }  
}

use sha2::{Sha256, Digest};
pub fn string_to_unique_id(input: &str) -> String {
    // Create a SHA-256 hasher
    let mut hasher = Sha256::new();

    // Feed the input string into the hasher
    hasher.update(input.as_bytes()); // Explicitly convert to bytes

    // Finalize the hash and get the result as a byte array
    let hash_result = hasher.finalize();

    // Convert the byte array to a hexadecimal string
    hash_result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
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
pub fn set_struct_json_path(path_str:&str){
    *DB_STRUCT_JSON_PATH.lock().unwrap() = String::from(path_str);
} 
pub fn get_struct_json_path() -> String {
    DB_STRUCT_JSON_PATH.lock().unwrap().clone()
}

lazy_static! {
    static ref DB_STRUCT_JSON_PATH: Mutex<String> = Mutex::new(String::from("./local_db/"));
}
#[macro_export]
macro_rules! auto_json_db_config {
    ($path_str:expr) => {
        use struct_to_json_db::*;
        use serde::{Deserialize, Serialize}; 
    };
}

#[macro_export]
macro_rules! auto_json_db_single{
    ($struct:ident) => {
        struct_to_json_db::paste! {
            impl [<$struct>] {
                pub fn save(&self) {
                    let file_path =get_struct_json_path() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(self).unwrap();
                    write_string_to_txt(&file_path, json_data);
                }
                pub fn load()->Self{
                    let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
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
                    let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(self).unwrap();
                    write_string_to_txt_encript(&file_path, json_data,$pass);
                }
                pub fn load()->Self{
                    let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
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
                        let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
                        let json_data = serde_json::to_string(&data).unwrap();
                        write_string_to_txt(&file_path, json_data);
                    }
                }
                pub fn remove(&self){
                    let mut data = Self::load();
                    let is_available = data.iter().any(|i| i == self);
                    if is_available {
                        data.retain(|i| i != self);
                        let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
                        let json_data = serde_json::to_string(&data).unwrap();
                        write_string_to_txt(&file_path, json_data);
                    }
                }
                pub fn save_all(data:&Vec<Self>){
                    let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
                    let json_data = serde_json::to_string(data).unwrap();
                    write_string_to_txt(&file_path, json_data);
                }
                pub fn load()->Vec<Self>{
                    let file_path = get_struct_json_path() + "/" + stringify!($struct) + ".json";
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

 

#[macro_export]
macro_rules! mapping_json_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field_name:ident : $field_type:ty),*
            $(,)?
        }
    ) => {
        paste! {
            $(#[$meta])*
            
            $vis struct [<Behalf $name>] {
                $($field_vis $field_name : $field_type),*
            }

            impl [<Behalf $name>] {
                // Implement the `load` function
                $vis fn to(&self) -> [<$name>] {
                    [<$name>] {
                        $($field_name: self.$field_name.clone()),*
                    }
                }
            }

            impl From<$name> for [<Behalf $name>] {
                fn from(item: $name) -> Self {
                    let now_idx = struct_to_json_db::unique_id(); 
                    [<Behalf $name>] {
                        idx: now_idx.0^now_idx.1,
                        created_at: now_idx.1,
                        $($field_name: item.$field_name),*
                    }
                }
                
            }

        }
    };
}