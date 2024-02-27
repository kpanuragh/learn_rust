fn main() {
    let x = 5;
    let y = 6;
    let z = add(x, y); // it copy value to function add
    println!("{} + {} = {}", x, y, z);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
