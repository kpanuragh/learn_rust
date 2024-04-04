pub fn apply_discount(price: f64, discount_percent: u8) -> f64 {
    price * (100.0 - discount_percent as f64) / 100.0
}
