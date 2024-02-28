**Rust Enums (Enumerations)**

Enums in Rust allow you to define a custom type that can hold one of several possible variants. This is incredibly helpful for representing values that have a limited set of distinct possibilities.

**Basic Example:**



```
enum IpAddrKind {
    V4,
    V6,
}

```

In this example, `IpAddrKind` can only be either `IpAddrKind::V4` or `IpAddrKind::V6`. This helps you model data more accurately within your code.

**Enums with Data**

Enums can hold additional data associated with each variant. There are various ways to do this:

-   **Struct-like Variants:**
    
    ```
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    ```
   
    

-   **Tuple-like Variants:**
    
    ```
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    ```
    

**Using Enums**

You often use `match` expressions to handle different enum variants:



```
let msg = Message::Move { x: 10, y: 5 };

match msg {
    Message::Quit => println!("Quitting!"),
    Message::Move {x, y} => println!("Moving to coordinates ({}, {})", x, y),
    Message::Write(text) => println!("Text message: {}", text),
}

```

**The Option Enum**

The `Option<T>` enum is part of Rust's standard library and is essential for handling potentially missing values. It has two variants:

-   **Some(T):** Indicates that a value of type `T` is present.
-   **None:** Indicates the absence of a value.

**Example:**


```
fn divide(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

```


The `divide` function returns `None` if there's an attempt to divide by zero, and `Some` with the result otherwise.

**Impl Blocks for Enums**

You can define methods and associated functions for your enums using `impl`:


```
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

```

**The 'if let' Construct**

The `if let` construct is a concise way to handle specific enum variants while ignoring others.

**Example:**


```
let some_number = Some(7);

if let Some(x) = some_number { // Match only the Some variant
    println!("The number is: {}", x); 
} 

```

**Let's Recap**

-   Enums define custom types with a limited set of variants.
-   Enums can carry associated data.
-   The `Option<T>` enum is for representing potential absence of a value.
-   Use `impl` to define methods and functions for your enums.
-   The `if let` provides a convenient way to work with enum patterns.

