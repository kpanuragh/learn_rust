
## Data Structures in Rust: Storing Values Effectively

This document explores three fundamental data structures in Rust for storing different types of data: Vectors, Strings, and Hash Maps. Understanding these structures is crucial for efficient data manipulation and organization in your Rust programs.

**1. Storing Lists of Values with Vectors**

-   **Vectors (Vec<T>)** are growable arrays that hold a collection of elements of the same type `T`. They are ideal for storing ordered lists of items, like:
    
    -   Scores in a game
    -   Lines of text in a file
    -   Elements of a shopping cart
-   **Key Features:**
    
    -   Dynamic size: Vectors can grow or shrink as needed at runtime.
    -   Random access: Elements can be accessed efficiently by their index.
    -   Various operations: You can push, pop, insert, and remove elements.
-   **Example:**
    

Rust

```
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector of integers
let mut names: Vec<String> = Vec::new(); // Empty vector for storing strings
names.push(String::from("Alice"));
names.push(String::from("Bob"));

```



**2. Storing UTF-8 Encoded Text with Strings**

-   **Strings (String)** are growable, mutable collections of characters representing UTF-8 encoded text. They are perfect for storing text data like:
    
    -   User input
    -   File contents
    -   Descriptive messages
-   **Key Features:**
    
    -   Mutable: Characters can be modified after creation.
    -   Owned data: Strings own the memory allocated for their characters.
    -   Slices: You can access portions of a string using string slices (`&str`).
-   **Example:**
    

Rust

```
let greeting = String::from("Hello, world!"); // Creates a string from a literal
let mut message = "Initial message".to_string(); // Converts a string literal to String
message.push_str(" This is an addition."); // Appends text to the string

```



**3. Storing Keys with Associated Values in Hash Maps**

-   **Hash Maps (HashMap<K, V>)** provide a way to store key-value pairs. Keys act as unique identifiers for accessing the associated values. Hash maps are useful for:
    
    -   User profiles (username as key, user data as value)
    -   Word definitions (word as key, definition as value)
    -   Configuration settings (setting name as key, value as value)
-   **Key Features:**
    
    -   Fast lookups: Keys are hashed for efficient retrieval of their corresponding values.
    -   Key uniqueness: Each key must be unique within the hash map.
    -   Different key and value types: Keys and values can be of different data types.
-   **Example:**
    

Rust

```
use std::collections::HashMap;

let mut colors: HashMap<&str, &str> = HashMap::new();
colors.insert("sky", "blue");
colors.insert("grass", "green");

println!("Color of sky: {}", colors.get("sky").unwrap()); // Access value by key

```



By mastering these data structures, you'll be well-equipped to manage various data types effectively in your Rust applications.

