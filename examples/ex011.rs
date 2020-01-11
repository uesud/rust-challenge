//! 与えられた整数をローマ数字に変換するサンプル

use rust_callenge::math::num_to_roman;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("n");
    println!("{}", num_to_roman(n));

}