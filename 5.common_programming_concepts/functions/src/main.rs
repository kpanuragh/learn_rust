fn main() {
    // let x = let y = 5; cant do this in rust because let is a statement not an expression
    let x = {
        let y = 5;
        y + 1 //it is an expression because it returns a value and you can see it is not ending with semicolon
    };
    println!("The value of x is: {}", x);
    greet();
    greet_with_name("Rust");
    println!("Sum: {}", add_return(2, 3));
    println!("Sum: {}", add(2, 3));
}
// Norml function example
fn greet() {
    println!("Hello, world!");
}
// function with parameters
fn greet_with_name(name: &str) {
    println!("Hello, {}", name);
}
//function with return value using return keyword
fn add_return(a: i32, b: i32) -> i32 {
    return a + b; //not best practice
}
// function without using return keyword
fn add(a: i32, b: i32) -> i32 {
    a + b
}
