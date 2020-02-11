//! 温度単位のサンプル

use rust_challenge::language_features::tempreture::Scale;

pub fn main() {
    let c = Scale::Celsius(10f64);
    let f = Scale::fahrenheit(&c);
    let k = Scale::kelvin(&c);

    println!("{}, {}, {}", c, f, k);
}