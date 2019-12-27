//! Ex1. 3または5で割り切れる正の整数の総和を求めるサンプルコード

use rust_callenge::math::sum_of_naturals_divisible_by_3_5;
use std::io::{stdin, stdout, Write};

pub fn main() {

    let mut stdout = stdout();
    stdout.write(b"Upper limit: ").unwrap();
    stdout.flush().unwrap();

    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();

    let n = input_text.trim().parse::<u32>()
        .expect("Invalid input. Upper limit must be positive integer.");

    let result = sum_of_naturals_divisible_by_3_5(n);
    println!("result = {}", result);
}