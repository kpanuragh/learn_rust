mod utils; //single module
mod models; //sub modules
mod controler; //nested modules
use models::product::Product;
fn main() {
    let product = Product {
        id: 1,
        name: String::from("Product 1"),
        price: 10.0,
    };
    println!("Product name: {}", product.name);
    println!("Product price: {}", product.price);
    println!("Product price after discount: {}", utils::apply_discount(product.price, 10));
    controler::sub_module::foo();

}
