//! 与えられた2つの整数の最小公倍数を求めるサンプルコード

use rust_challenge::math::lcm;

mod common;
use common::read_u32;

pub fn main() {
    let a = read_u32("value1");
    let b = read_u32("value2");
    
    let result = lcm(a, b);
    println!("result = {}", result);
}