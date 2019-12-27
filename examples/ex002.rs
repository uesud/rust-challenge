//! 与えられた2つの整数の最大公約数を求めるサンプルコード

use rust_callenge::math::gcd;

// 標準入力から数値を読み取る
fn read_value(s: &str) -> u32 {
    use std::io::{stdin, stdout, Write};

    let mut stdout = stdout();
    stdout.write(s.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut input_text = String::new();    
    stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse::<u32>()
        .expect("Invalid input. Input value must be positive integer.")
}

pub fn main() {

    let a = read_value("value1: ");
    let b = read_value("value2: ");
    
    let result = gcd(a, b);
    println!("result = {}", result);
}