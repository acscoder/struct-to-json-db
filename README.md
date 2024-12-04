struct-to-json-db is a minimalist file-based database designed for straightforward software applications. It leverages the Serde crate to seamlessly serialize and deserialize a HashMap of structured data to and from JSON files, ensuring simplicity and ease of use.

# Example code 
For a practical example, check out the [example code repository](https://github.com/acscoder/struct-to-json-db-test).

### Get started 
To get started, add the crate to your `Cargo.toml` file:
```
[dependencies]
struct-to-json-db = {  path = "https://github.com/acscoder/struct-to-json-db.git" } 
```
or 
```
[dependencies]
struct-to-json-db = "x.x.x" 
```
Check the latest version on https://crates.io/search?q=struct-to-json-db

### Configuration
To configure struct-to-json-db, use the auto_json_db_config! macro to specify the directory where your JSON files will be stored:
```rust
struct_to_json_db::auto_json_db_config!("./db/");
```
Note: Ensure that the path ends with a slash (/). For example, "./db/" is correct, but "./db" is not.
 
### Adding the Macro to Your Struct
Use the #[auto_json_db] macro for your struct to automatically add a unique ID and additional methods:
```rust
use struct_to_json_db::*;
auto_json_db_config!("./db/");

#[auto_json_db(unique="title")]
struct Post {
    title: String,
    description: String,
    categories: Vec<u64>
}
#[auto_json_db(unique="name")]
struct Category {
    name: String
}
json_db_relation!(Post=categories, Category);
 
fn main() {
    let mut all_posts = Post::get_all(); 
    println!("{:?}", all_posts);
      
    let c1 = Category::new("cat_1".to_owned());
    let c2 =  Category::new("cat_2".to_owned());
    c1.save();
    c2.save();
   
    let c = add_posts("post_2".to_owned(), "desc_1".to_owned(), &vec![c1,c2]);
    println!("{:?}", c);
    
}
fn add_posts(title: String, description: String,cates:&Vec<Category>)->Option<u64> {
    let mut p1 = Post::new( title,description,vec![]);
    if cates.len() > 0{
        p1.set_categories(cates);
    }
    p1.save()
}
```
This macro will add the following:

A unique id (idx: u64) to the Post struct.

A `new` method for creating new instances.

A `get_all` method to retrieve all saved posts from the JSON file as a HashMap with idx as the key and Post object as the value.

A `save` method to save a single Post to the JSON file.

A `save_vec` method to save a vector of Post objects to the JSON file.

A `get_by_id` method to retrieve a Post by its idx.

A `remove_by_id` method to remove a Post by its idx.

A `clear` method to remove all Post objects.

### Contributing
Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/acscoder/struct-to-json-db).

### License
This project is licensed under the [APACHE License](https://github.com/acscoder/struct-to-json-db/LICENSE-2.0.txt).