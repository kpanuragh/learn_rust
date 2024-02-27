fn main() {
    let x = (1,2,3,4,5); //same data or diffrent data but fxed size
    println!("x.0 = {}", x.0); //access the data using index
    println!("x.1 = {}", x.1);
    let x = ("hello",1,true,3.13); //different data type
    println!("Different data type.............");
    println!("x.0 = {}", x.0);
    println!("x.1 = {}", x.1);
    println!("x.2 = {}", x.2);
    println!("x.3 = {}", x.3);
    println!("x = {:?}",x);
}
