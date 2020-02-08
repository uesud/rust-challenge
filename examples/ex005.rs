//! 与えられた整数より小さな素数からセクシー素数の組を返すサンプルコード

use rust_challenge::math::sexy_prime_pair;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("Upper limit");

    for (p1, p2) in sexy_prime_pair(n) {
        println!("({}, {})", p1, p2);
    }
}