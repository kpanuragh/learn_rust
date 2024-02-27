
**Tuples**

-   **Definition:** Tuples are ordered collections of elements that can have different data types. They are like fixed-length arrays, but the elements don't have to be of the same type.
    
-   **Syntax:**
    

    
    ```
    let my_tuple = (10, "hello", 3.14);
    
    ```
    

    

-   **Accessing Elements:** Use dot notation and the index (starting from 0):
    

    ```
    println!("First element: {}", my_tuple.0); 
    println!("Third element: {}", my_tuple.2); 
    
    ```
    

    

-   **Deconstructing:** You can unpack tuple values into individual variables:

    
    ```
    let (number, text, pi) = my_tuple;
    
    ```
    
    
    

**Structs**

-   **Definition:** Structs are user-defined composite data types. They group related data fields together under a single name.
    
-   **Syntax:**
    

    
    ```
    struct Point {
        x: i32,
        y: i32,
    }
    
    ```
    
    

-   **Creating Instances:**
    
    
    ```
    let origin = Point { x: 0, y: 0 };
    
    ```
    
    

-   **Accessing Fields:** Use dot notation:
    

    
    ```
    println!("Origin coordinates: ({}, {})", origin.x, origin.y);
    
    ```
    

