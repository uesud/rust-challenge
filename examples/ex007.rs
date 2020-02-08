//! 1,000,000より小さいすべての友愛数の組を出力するサンプル

use rust_challenge::math::amicable_numbers;

pub fn main() {
    for (p1, p2) in amicable_numbers(1_000_000) {
        println!("({}, {})", p1, p2);
    }
}