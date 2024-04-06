//TODO: Learn about vector

fn main() {
    //intialize a vector
    let mut v: Vec<i32> = Vec::new();
    // Vector implemented using generics
    // above vector has type i32
    let mut another_vec = vec![1, 2, 3, 4, 5];
    // Vector initialized with values
    // vec! macro is used to create a vector
    //Update a vector
    v.push(5);
    another_vec.push(6);
    //Accessing elements of a Vector
    //using index
    let third: &i32 = &another_vec[2];
    println!("The third element is {}", third);
    //using get method
    match another_vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    //Iterating over the values in a vector
    for i in &another_vec {
        println!("{}", i);
    }
    //Iterating over the values in a vector and changing the values
    for i in &mut another_vec {
        *i += 50; // * is used to dereference the value
    }
    //Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
