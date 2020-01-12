//! 1,000,000までの数のうち、(最長コラッツ数列になる数, その数列の長さ) の組を返す

use rust_callenge::math::max_len_of_collatz_1_000_000;

pub fn main() {
    let (n, len) = max_len_of_collatz_1_000_000();
    println!("n={}, len={}", n, len);

}