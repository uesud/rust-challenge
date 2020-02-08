//! 任意個数の要素から最小値を求めるマクロのサンプル

use rust_challenge::my_min;

pub fn main() {
    println!("min(1,2,3,4) -> {}", my_min!(1,2,3,4));
}