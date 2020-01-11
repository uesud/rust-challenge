//! 5bitの数値→グレイコード→グレイコード復号を出力するサンプル

use rust_callenge::math::{binary_to_gray, gray_to_binary};

pub fn main() {
    println!("Number,\tBinary,\tGray,\tDecorded");
    for i in 0..=0b11111 {
        let g = binary_to_gray(i);
        let b = gray_to_binary(g);
        println!("{},\t{:>05b},\t{:>05b},\t{:>05b}", i, i, g, b);
    }
}