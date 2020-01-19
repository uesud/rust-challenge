//! Example用のユーティリティ

/// 標準入力から数値(u32)を読み取る
#[allow(dead_code)]
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

/// 標準入力から文字列を読み取る
#[allow(dead_code)]
pub fn read_string(s: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut stdout = stdout();
    stdout.write((s.to_string() + ": ").as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut input_text = String::new();    
    stdin().read_line(&mut input_text).unwrap();

    // 改行文字を削ってから返す
    input_text.replace("\n", "")
}

use rust_callenge::language_features::{Ipv4, ParseIpv4Error};

/// 標準入力からIPv4(ドット形式)を読み取る
/// 失敗したらエラーメッセージを表示する
#[allow(dead_code)]
pub fn read_ipv4(s: &str) -> Option<Ipv4> {
    let ipv4_str = read_string(s);
    match Ipv4::parse(&ipv4_str) {
        Ok(ipv4) => Some(ipv4),
        Err(ParseIpv4Error::InvalidValue(s)) => {
            println!("Invalid value: {}", s);
            None
        },
        Err(ParseIpv4Error::InvalidFormat) => {
            println!("Invalid input");
            None
        },
    }
}
