fn main() {
    //&str is a string slice and it is a reference to a string
    let name = "Rust"; // str slice  no ownserhip just reference to the string stored in any location
    let name_sample = name; // no ownership transfered
    // use _ to ignore it
    let name_string = String::from("John");
    let name_string_sample = name_string; // ownership transfer to name_string_sample . name_string is no longer valid
    let name_string_sample_clone = name_string_sample.clone(); // copying the value of name_string_sample to name_string_sample_clone
    greet_rust(name); // passing the reference to the function
    println!("Goodbye {name}"); // main still has ownership of name
    println!("Complex String example.................");
    greet_string(name_string_sample_clone); // ownership transfer to the function
    println!("Name already droped");
    // println!("Goodbye {name_string_sample_clone}"); cannot use name as its ownership has been transferred to greet_string
    // Pass by reference
    println!("Pass by reference....................");
    greet_refrence_string(&name_string_sample); // passing the reference to the function
    println!("Name still available {name_string_sample}");
}
fn greet_rust(name: &str) {
    println!("Hello {name}");
}
fn greet_string(name:String){ //ownership of the string is transferred to the function
    println!("Hello {name}");  
} //droped name variable when the scope ends
fn greet_refrence_string(name:&String){ // only transfer the refrerence to the function
    println!("Hello {name}");  
} // ownserhip still with the main function as it was not transferred
