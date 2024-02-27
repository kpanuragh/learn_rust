struct Rectangle { //custom data type with different fields
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 { //function to calculate area
    rectangle.width * rectangle.height
}
fn main() {
    let mut rect = Rectangle { //creating instance of Rectangle
        width: 30,
        height: 50,
    };
    rect.width = 100 ; // we can acess and modify the fields of the instance using key if the instance is mutable
    println!("The area of the rectangle is {} square ", area(&rect)); //calling the area function
}
