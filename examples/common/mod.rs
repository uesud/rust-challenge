//! Example用のユーティリティ

/// 標準入力から数値(T型)を読み取る
fn read_num<T>(s: &str) -> Result<T, T::Err>
    where T: std::str::FromStr {

    use std::io::{stdin, stdout, Write};

    let mut stdout = stdout();
    stdout.write((s.to_string() + ": ").as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut input_text = String::new();    
    stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse::<T>()
}

/// 標準入力から数値(u8)を読み取る
#[allow(dead_code)]
fn read_u8(s: &str) -> u8 {
    read_num::<u8>(s)
        .expect(&format!("Invalid input. {} must be unsigned 8bit number.", s))
}

/// 標準入力から数値(u32)を読み取る
#[allow(dead_code)]
pub fn read_u32(s: &str) -> u32 {
    read_num::<u32>(s)
        .expect(&format!("Invalid input. {} must be unsigned 32bit number.", s))
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

use rust_challenge::language_features::{Ipv4, ParseIpv4Error};

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
