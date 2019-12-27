//! Example用のユーティリティ

/// 標準入力から数値(u32)を読み取る
pub fn read_u32(s: &str) -> u32 {
    use std::io::{stdin, stdout, Write};

    let mut stdout = stdout();
    stdout.write((s.to_string() + ": ").as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut input_text = String::new();    
    stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse::<u32>()
        .expect(&format!("Invalid input. {} must be positive integer.", s))
}