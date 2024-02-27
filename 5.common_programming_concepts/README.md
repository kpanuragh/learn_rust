
### 1. Basic Types

Rust offers a variety of fundamental data types to represent different kinds of values:

-   **Integers:** These represent whole numbers, signed (`i8`, `i16`, etc.) or unsigned (`u8`, `u16`, etc.), with sizes ranging from 8 to 128 bits.
-   **Floating-point numbers:** These represent real numbers with decimals, using either `f32` (single-precision) or `f64` (double-precision) for different levels of precision.
-   **Boolean:** This type represents a logical value, either `true` or `false`.
-   **Character:** This type represents a single character, denoted by a single quote (e.g., `'a'`).

### 2. Variables and Mutability

Variables store data in memory and can be referenced by name in your code. Rust enforces strong ownership rules over variables, ensuring memory safety.

-   **Variable declaration:** You declare variables using `let` followed by the variable name, data type, and an optional initial value:


```
let age: u32 = 30; // Declares an unsigned integer variable named 'age' with initial value 30

```

-   **Mutability:** By default, variables are immutable, meaning their values cannot be changed after assignment. Use `mut` to allow modification:


```
let mut count: i32 = 0;
count += 1; // This is allowed since 'count' is mutable

```
-   **Const:** In Rust, a **`const`** defines a **fixed, unchanging value** known at compile time. It has the following characteristics:

	-   ***Immutability:*** The value cannot be modified after being declared.
	-   ***Compile-time knowledge:*** The value must be known at compile time, meaning calculations or function calls are not allowed within the definition.
	-   ***Type annotation:*** The type of the constant must be explicitly specified.

Here's an example:


```
const MAX_VALUE: u32 = 100; // Constant named 'MAX_VALUE' with type 'u32' and value 100

fn main() {
    println!("Maximum value: {}", MAX_VALUE);
}

```

In this example, `MAX_VALUE` is a constant with the value 100 and cannot be changed throughout the program.

### 3. Functions

Functions are reusable blocks of code that perform specific tasks. They take zero or more arguments as inputs and can optionally return a value.

-   **Function definition:**


```
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

```

-   The first line defines a function named `greet` that takes a reference to a string (`&str`) as input and prints a greeting message.
-   The second line defines a function named `add` that takes two integers (`i32`) as input, adds them, and returns the result as an integer.

### 4. Control Flow

Control flow statements dictate how your program executes by allowing conditional branching and looping.

-   **Conditional statements:**
    -   `if` statements execute code blocks based on a boolean condition:


```
if age >= 18 {
    println!("You are eligible to vote.");
} else {
    println!("You are not eligible to vote.");
}

```

```
* `else if` and `else` can be chained for more complex conditions.

```

-   **Loops:**
    -   `for` loops iterate over a collection, executing code for each item:


```
for i in 0..5 {
    println!("Number: {}", i);
}

```

```
* `while` loops execute code repeatedly as long as a condition is true:

```


```
let mut count = 0;
while count < 10 {
    println!("Count: {}", count);
    count += 1;
}

```

### 6. Comments

Comments are lines of text ignored by the compiler, used to explain your code and improve readability.

-   Single-line comments start with `//`.
-   Multi-line comments use `/*` and `*/`

