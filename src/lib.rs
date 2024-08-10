pub use struct_to_json_db_macro::auto_json_db; 
use std::fs;
use std::io::Write; 
use rand::Rng;
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
pub fn unique_id() -> (u64,u64) {
    let start = std::time::SystemTime::now();
    let timestamp = start.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64 ;
    let random_number = rand::thread_rng().gen::<u64>();
    (random_number,timestamp)
}

#[macro_export]
macro_rules! auto_json_db_config {
    ($path_str:expr) => {
        use struct_to_json_db::auto_json_db;
        use serde::{Deserialize, Serialize};
        static DB_STRUCT_JSON_PATH:&str = $path_str;
    };
}
#[macro_export]
macro_rules! struct_db_relation {
    ($first:ident, $second:ident) => {
        struct_to_json_db::paste! {
            #[auto_json_db]
            pub struct [<$first $second Relation>] {
                pub [<$first _ id>]:u64,
                pub [<$second _ id>]:u64,
                pub weight:f32,
                pub name:String
            }
            
        }
    };
}
#[macro_export]
macro_rules! db_relation_add {
    ($first:ident=$first_val:literal, $second:ident=$second_val:literal) => {
        struct_to_json_db::paste! {
            [<$first $second Relation>]::new($first_val,$second_val, 0.0, "".to_string())
        }
    };
    ($first:ident=$first_val:expr, $second:ident=$second_val:expr, weight=$weight_val:expr) => {
        struct_to_json_db::paste! {
            [<$first $second Relation>]::new($first_val,$second_val,  $weight_val , "".to_string() ) 
        }
    };
    ($first:ident=$first_val:expr, $second:ident=$second_val:expr, name=$name_val:expr) => {
        struct_to_json_db::paste! {
            [<$first $second Relation>]::new($first_val,$second_val, 0.0, $name_val ) 
        }
    };
    ($first:ident=$first_val:expr, $second:ident=$second_val:expr, name=$name_val:expr,weight=$weight_val:expr) => {
        struct_to_json_db::paste! {
            [<$first $second Relation>]::new($first_val,$second_val, $weight_val, $name_val ) 
        }
    };
}
#[macro_export]
macro_rules! db_relation_get {
    ($first:ident=$first_val:literal, $second:ident) => {
        struct_to_json_db::paste! {
            {
                let all_relation = [<$first $second Relation>]::get_all();
                let ret:Vec<(u64,[<$first $second Relation>])> = all_relation.into_iter().filter(|x| x.1.[<$first _ id>] == $first_val).collect(); 
                ret
            }
        }
    };
    ($first:ident, $second:ident=$second_val:literal) => {
        struct_to_json_db::paste! {
            {
                let all_relation = [<$first $second Relation>]::get_all();
                let ret:Vec<(u64,[<$first $second Relation>])> = all_relation.into_iter().filter(|x| x.1.[<$second _ id>] == $second_val).collect();
                ret
            
            }
        }
    };
     
}
#[macro_export]
macro_rules! db_relation_chunking {
    ($first:ident, $second:ident) => {
        struct_to_json_db::paste! {
            #[derive(Serialize,Deserialize,Clone,Debug)]
            pub struct [<$first $second Chunking>] {
                pub idx:u64,
                pub data:Vec<$second>
            }
            impl [<$first $second Chunking>] {
                pub fn new(idx:u64,data:Vec<$second>) -> Self {
                    Self {
                        idx,
                        data 
                    }
                }
                pub fn get(idx:u64) -> Option<Self> {
                    let file_uri = format!("{}/{}{}Chunking_{}.json",DB_STRUCT_JSON_PATH,stringify!($first),stringify!($second), idx);
                    let json_str = struct_to_json_db::read_string_from_txt(&file_uri);
                    if json_str.is_empty(){
                        return None;
                    }
                    let obj = serde_json::from_str(&json_str);
                    obj.ok()
                }
                pub fn save(&self) {
                    let json_str = serde_json::to_string(&self).unwrap();
                    struct_to_json_db::write_string_to_txt(&format!("{}/{}{}Chunking_{}.json",DB_STRUCT_JSON_PATH,stringify!($first),stringify!($second), self.idx), json_str);
                }
            }
        }
    };
}  

#[macro_export]
macro_rules! db_relation_chunking_add {
    ($first:ident=$first_val:literal, $second:ident=$second_val:expr) => {
        struct_to_json_db::paste! {
            {
                let obj = [<$first $second Chunking>]::get($first_val); 
                if obj.is_none() {
                    let obj = [<$first $second Chunking>]::new($first_val,$second_val);
                    obj.save();
                    return obj;
                } else {
                    let mut obj = obj.unwrap();
                    obj.data.extend($second_val);
                    obj.save();
                    return obj;
                }   
            }
        }
    };
}

#[macro_export]
macro_rules! db_relation_chunking_get {
    ($first:ident=$first_val:literal, $second:ident) => {
        struct_to_json_db::paste! {
            [<$first $second Chunking>]::get($first_val)
        }
    };
}