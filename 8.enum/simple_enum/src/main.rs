enum IPAddrKind {
    V4,
    V6,
}
enum IPAddrKindV2 {
    V4(String),
    V6(String),
}
enum IPAddrKindV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    let home = IPAddrKindV2::V4(String::from("127.0.0.1"));
    let home_v6 = IPAddrKindV2::V6(String::from("::1"));
    let home_v4 = IPAddrKindV3::V4(127, 0, 0, 1);
    let home_v6 = IPAddrKindV3::V6(String::from("::1"));
}
