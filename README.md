# struct-to-json-db

[![Crates.io](https://img.shields.io/crates/v/struct-to-json-db.svg)](https://crates.io/crates/struct-to-json-db)
[![Build Status](https://github.com/acscoder/struct-to-json-db/workflows/CI/badge.svg)](https://github.com/acscoder/struct-to-json-db/actions)
[![License](https://img.shields.io/crates/l/struct-to-json-db.svg)](https://github.com/acscoder/struct-to-json-db/blob/main/LICENSE-2.0.txt)

**struct-to-json-db** is a minimalist, file-based database tailored for straightforward Rust applications. Leveraging the power of the [Serde](https://serde.rs/) crate, it effortlessly serializes and deserializes structured data stored as JSON files. This ensures simplicity, flexibility, and ease of integration for developers seeking a lightweight database solution without the overhead of setting up complex systems.

## Table of Contents

- [Features](#features)
- [Example Code](#example-code)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [Usage](#usage)
  - [Adding the Macro to Your Struct](#adding-the-macro-to-your-struct)
  - [Defining Relationships](#defining-relationships)
  - [Encryption Support](#encryption-support)
- [API Overview](#api-overview)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Automatic ID Generation:** Automatically generate unique identifiers for your structs.
- **Serialization & Deserialization:** Seamlessly convert structs to and from JSON files.
- **Singleton Support:** Manage singleton configurations effortlessly.
- **Unique Keys:** Enforce uniqueness on specified fields to maintain data integrity.
- **Big Size Handling:** Split large datasets into multiple JSON files for efficient data management.
- **Data Encryption:** Protect sensitive data using environment-based encryption keys.
- **Relationship Management:** Define and manage relationships between different data structs.

## Example Code

For a practical example, visit the [example code repository](https://github.com/acscoder/struct-to-json-db-test).

## Getting Started

### Installation

To integrate **struct-to-json-db** into your project, add it to your `Cargo.toml`:

```toml
[dependencies]
struct-to-json-db = "x.x.x"  # Replace x.x.x with the latest version
```

Alternatively, you can specify the GitHub repository directly:

```toml
[dependencies]
struct-to-json-db = { git = "https://github.com/acscoder/struct-to-json-db.git" }
```

> **Note:** Always check [crates.io](https://crates.io/crates/struct-to-json-db) for the latest version.

### Configuration

Before using the database, configure the directory where your JSON files will be stored:

```rust
use struct_to_json_db::set_struct_json_path;

fn main() {
    set_struct_json_path("./db/"); // Ensure the path ends with a slash (/)
}
```

> **Important:** The path must end with a slash (`/`). For example, `./db/` is correct, whereas `./db` is incorrect.

## Usage

### Adding the Macro to Your Struct

Use the `#[auto_json_db]` attribute macro to automatically add functionalities such as unique ID generation and data management methods to your structs.

#### Basic Usage

```rust
use struct_to_json_db::auto_json_db;
use serde::{Deserialize, Serialize};

#[auto_json_db]
#[derive(Serialize, Deserialize, Debug)]
struct YourStruct {
    // Your fields here
}
```

#### With Unique Key

Enforce uniqueness on specific fields by specifying the `unique` attribute:

```rust
use struct_to_json_db::auto_json_db;
use serde::{Deserialize, Serialize};

#[auto_json_db(unique = "your_unique_key")]
#[derive(Serialize, Deserialize, Debug)]
struct YourStruct {
    // Your fields here
}
```

#### Handling Large Structs

For structs with large datasets, use the `bigsize` attribute to split data into multiple files, enhancing performance and manageability:

```rust
use struct_to_json_db::auto_json_db;
use serde::{Deserialize, Serialize};

#[auto_json_db(bigsize, unique = "your_unique_key")]
#[derive(Serialize, Deserialize, Debug)]
struct YourStruct {
    // Your fields here
}
```

#### Singleton Structs

Manage singleton configurations using the `singleton` attribute:

```rust
use struct_to_json_db::auto_json_db;
use serde::{Deserialize, Serialize};

#[auto_json_db(singleton)]
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    // Configuration fields
}
```

### Defining Relationships

Establish relationships between different structs using the `json_db_relation!` macro. This facilitates relational data management, akin to foreign keys in traditional databases.

```rust
use struct_to_json_db::json_db_relation;

// Example: One Post has many Categories
json_db_relation!(Post = categories, Category);

// Example: One-to-One Relationship
json_db_relation!(Post = categories, Category, "1-1");
```

### Encryption Support

Protect sensitive data by encrypting JSON files. Define an environment variable as your encryption key and specify it in the macro:

1. **Set Up Environment Variable:**

   Create a `.env` file in your project root:

   ```env
   APP_SECRET_KEY=your_secret_key
   ```

2. **Load Environment Variables:**

   Use the `dotenv` crate to load the `.env` file:

   ```rust
   use dotenv::dotenv;

   fn main() {
       dotenv().ok();
       // Your code here
   }
   ```

3. **Apply Encryption in Macro:**

   ```rust
   use struct_to_json_db::auto_json_db;
   use serde::{Deserialize, Serialize};
   use dotenv::dotenv;

   #[auto_json_db(encript = "APP_SECRET_KEY")]
   #[derive(Serialize, Deserialize, Debug)]
   struct SensitiveData {
       // Sensitive fields
   }
   ```

> **Note:** The `encript` attribute requires the specified environment variable (`APP_SECRET_KEY` in this case) to be set. It uses this key to encrypt and decrypt your JSON data.

## API Overview

The `#[auto_json_db]` macro enriches your structs with a suite of methods to facilitate data management:

- **Unique ID (`idx`):** Automatically adds a unique identifier (`u64` or `String`) to the struct.
- **Constructor (`new`):** Creates new instances of the struct.
- **CRUD Operations:**
  - `get_all()`: Retrieves all saved instances as a `HashMap<idx, Struct>`.
  - `save()`: Saves a single instance to the JSON file.
  - `save_vec(v: Vec<Struct>)`: Saves multiple instances at once.
  - `get_by_id(id: idx)`: Fetches an instance by its unique ID.
  - `remove_by_id(id: idx)`: Removes an instance by its unique ID.
  - `clear()`: Deletes all instances from the JSON file.
- **Relationship Methods:** Manage relations between different structs as defined by `json_db_relation!`.
- **Encryption Handling:**
  - `set_data_string(file_path, db_string)`: Encrypts and writes data if encryption is enabled.
  - `get_data_string(file_path)`: Decrypts and reads data if encryption is enabled.

## Example

Below is a comprehensive example demonstrating the usage of **struct-to-json-db** with encryption, unique keys, big size handling, and relationships.

```rust
use struct_to_json_db::*;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[auto_json_db(singleton, encript = "APP_SECRET_KEY")]
#[derive(Serialize, Deserialize, Debug)]
struct SiteConfig {
    url: String,
    limit: i32,
}

impl SiteConfig {
    fn default() -> Self {
        Self::new("".to_owned(), 0)
    }
}

#[auto_json_db(bigsize, unique = "title")]
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    description: String,
    categories: Vec<u64>,
}

#[auto_json_db(unique = "name")]
#[derive(Serialize, Deserialize, Debug)]
struct Category {
    name: String,
}

json_db_relation!(Post = categories, Category);

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Set the directory for JSON database files
    set_struct_json_path("./db/");

    // Load the singleton SiteConfig
    let mut config = SiteConfig::load();
    println!("Site Config: {:?}", config);

    // Retrieve all posts
    let all_posts = Post::get_all();
    println!("Number of posts: {}", all_posts.len());

    // Create and save categories
    let category1 = Category::new("Technology".to_owned());
    let category2 = Category::new("Health".to_owned());
    category1.save();
    category2.save();

    // Add a new post with assigned categories
    let post_id = add_post(
        "Rust Programming".to_owned(),
        "An introduction to Rust.".to_owned(),
        &vec![category1, category2],
    );
    println!("New Post ID: {:?}", post_id);
}

fn add_post(title: String, description: String, categories: &Vec<Category>) -> Option<u64> {
    let mut post = Post::new(title, description, vec![]);
    if !categories.is_empty() {
        post.set_categories(categories);
    }
    post.save()
}
```

### Explanation

1. **SiteConfig Struct:**
   - Marked as a singleton with encryption enabled.
   - Automatically loads and saves the configuration to `./db/SiteConfig.json`.
   
2. **Post Struct:**
   - Configured with `bigsize` to handle large datasets by splitting into multiple files.
   - Enforces uniqueness on the `title` field.
   
3. **Category Struct:**
   - Enforces uniqueness on the `name` field.

4. **Relationships:**
   - Defined a one-to-many relationship where a `Post` can have multiple `Categories`.
   
5. **Main Function:**
   - Initializes environment variables and sets the database path.
   - Loads the singleton configuration.
   - Retrieves and prints the number of posts.
   - Creates and saves new categories.
   - Adds a new post associated with the created categories.

## Contributing

Contributions are highly encouraged! Whether it's reporting bugs, suggesting features, or submitting pull requests, your input helps make **struct-to-json-db** better for everyone.

1. **Fork the Repository:** Click the [Fork](https://github.com/acscoder/struct-to-json-db/fork) button at the top right of the repository page.

2. **Clone Your Fork:**

   ```bash
   git clone https://github.com/your-username/struct-to-json-db.git
   cd struct-to-json-db
   ```

3. **Create a Feature Branch:**

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Commit Your Changes:**

   ```bash
   git commit -m "Add your message here"
   ```

5. **Push to Your Fork:**

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request:** Navigate to your fork on GitHub, switch to your feature branch, and click the **New Pull Request** button.

## License

This project is licensed under the [Apache License 2.0](https://github.com/acscoder/struct-to-json-db/blob/main/LICENSE-2.0.txt). You are free to use, modify, and distribute this software as per the terms of the license.

---

**Happy Coding!** 🚀