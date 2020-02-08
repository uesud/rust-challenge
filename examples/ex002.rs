//! 与えられた2つの整数の最大公約数を求めるサンプルコード

use rust_challenge::math::gcd;

mod common;
use common::read_u32;

pub fn main() {
    let a = read_u32("value1");
    let b = read_u32("value2");
    
    let result = gcd(a, b);
    println!("result = {}", result);
}