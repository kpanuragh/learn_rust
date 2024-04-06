//NOTE: lets learn about string in rust
fn main() {
    //NOTE: String is a growable, heap-allocated data structure in Rust
    //NOTE: String is a wrapper over a Vec<u8> and it is UTF-8 encoded
    //NOTE: String is a mutable, growable, owned, heap-allocated data structure
    //NOTE: String is a UTF-8 encoded, growable, heap-allocated data structure
    //NOTE: String is a growable, heap-allocated, UTF-8 encoded data structure
    //NOTE: String is a heap-allocated, growable, UTF-8 encoded data structure
    //NOTE: create a new empty string
    let mut s = String::new();
    //NOTE: create a new string with initial value
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    //NOTE: updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
