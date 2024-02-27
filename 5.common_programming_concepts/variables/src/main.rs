const MAX_POINTS: u32 = 100_000;
// let max = 4000; can't declare a variable here using let 
fn main() {
    let x = 5 ;
    println!("The value of x is: {}", x);
    // x = 6; cant change x because it is immutable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // y is mutable
    println!("The value of y is: {}", y);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // let MAX_POINTS = 200_000; cant deine MAX_POINTS
    let z = 6;  
    let z = z + 1; // variable shadowing  thought the value of z is 7 now
    println!("The value of z is: {}", z);
}
