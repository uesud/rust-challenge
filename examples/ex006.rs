//! 与えられた整数より小さな過剰数のリストを返すサンプルコード

use rust_challenge::math::abundant_numbers;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("Upper limit");

    for a in abundant_numbers(n) {
        println!("{}", a);
    }
}