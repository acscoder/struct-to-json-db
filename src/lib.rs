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
macro_rules! json_db_one_many {
    ($first:ident, $second:ident) => {
        struct_to_json_db::paste! {
            #[derive(Serialize,Deserialize,Clone,Debug)]
            pub struct [<$first $second OneMany>] {
                pub idx:u64,
                pub data:Vec<[<$second>]>
            }
            impl [<$first $second OneMany>] {
                pub fn new(idx: u64, data: Vec<[<$second>]>) -> Self {
                    Self { idx, data }
                }
                pub fn add(&mut self, new_data: Vec<[<$second>]>) {
                    self.data.extend(new_data);
                }
                pub fn get_path()->String{
                    format!("{}/one_many_{}_{}.json", DB_STRUCT_JSON_PATH, stringify!($first), stringify!($second))
                }
                pub fn get_all()->std::collections::HashMap<u64,Self>{
                    let file_path = Self::get_path();
                    let db_string = read_string_from_txt(&file_path);
                    serde_json::from_str(&db_string).unwrap_or_default() 
                }
                pub fn save(&self){ 
                    let mut db = Self::get_all();
                    db.insert(self.idx, self.clone());
                    Self::save_all(&db);
                }
                pub fn save_all(db:&std::collections::HashMap<u64,Self>){
                    let file_path = Self::get_path();
                    let db_string = serde_json::to_string(db).unwrap();
                    struct_to_json_db::write_string_to_txt(&file_path, db_string);
                }
                pub fn remove(&self){
                    Self::remove_by_id(self.idx);
                }
                pub fn remove_by_id(id: u64){ 
                    let mut db = Self::get_all(); 
                    db.remove(&id);
                    Self::save_all(&db); 
                }
                pub fn remove_by_ids(ids: &Vec<u64>){
                    let mut db = Self::get_all(); 
                    for id in ids{
                        db.remove(&id);
                    }
                    Self::save_all(&db); 
                }
                pub fn clear(){
                    let file_path = Self::get_path();
                    struct_to_json_db::write_string_to_txt(&file_path, "".to_owned());
                }
            }
        }
    };
}
#[macro_export]
macro_rules! json_db_one_many_get{
    ($first:ident, $second:ident) => {
        struct_to_json_db::paste! {
            {
                [<$first $second OneMany>]::get_all()
            }
        }
    };
}
#[macro_export]
macro_rules! json_db_one_many_add{
    ($first:ident=$first_val:literal, $second:ident=$second_val:literal) => {
        struct_to_json_db::paste! {
            {
               let all_data = [<$first $second OneMany>]::get_all();
               let newone = [<$first $second OneMany>]::new($first_val,$second_val);
               all_data.insert(newone.idx, newone);
               [<$first $second OneMany>]::save_all(&all_data);
               all_data
            }
        }
    };
}
#[macro_export]
macro_rules! json_db_many_many {
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
macro_rules! json_db_many_many_add {
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
macro_rules! json_db_many_many_get {
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
 