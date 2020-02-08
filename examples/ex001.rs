//! Ex1. 3または5で割り切れる正の整数の総和を求めるサンプルコード

use rust_challenge::math::sum_of_naturals_divisible_by_3_5;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("Upper limit");
    let result = sum_of_naturals_divisible_by_3_5(n);
    println!("result = {}", result);
}