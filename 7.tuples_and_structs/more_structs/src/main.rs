struct Rectangle { //simple struct  
    width: u32,
    height: u32,
}
impl Rectangle { //but we ca use like class in other lanaguages we can use impl to define methods
    //for the struct
    fn area(&self) -> u32 { // here the self is the instance of the struct  
        self.width * self.height
    }
    fn new(width:u32,height:u32) -> Rectangle { // we can also define static methods like other
        // languages without passing self
        Rectangle {
            width, //same like width:width
            height,
        }
    }
}
struct  SampleTuple(i32,i32); // we can also define tuple struct without using struct name
fn main() {
    let rect1 = Rectangle { //creating a instance of the struct
        width: 30,
        height: 50,
    }; 
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle::new(20,30); //calling the static method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    let tuple = SampleTuple(10,20); //creating a instance of the tuple struct
    println!("{:?}",tuple.0); 
    println!("{:?}",tuple.1); 
    
}
