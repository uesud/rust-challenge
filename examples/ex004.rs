//! 与えられた整数より小さな最大の素数を返すサンプルコード

use rust_challenge::math::largest_prime;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("Upper limit");
    
    match largest_prime(n) {
        Some(x) => println!("result = {}", x),
        None => println!("None"),
    }    
}