/// 数値列([u8])と16進文字列表記を相互に変換するサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::{bytes_to_string};

pub fn main() {
    let bytes: Vec<u8> = read_string("8 bit numbers, separated by comma")
        .split(',')
        .collect::<Vec::<&str>>()
        .into_iter()
        .map(|w| 
            w.trim()
            .parse::<u8>()
            .expect(&format!("invalid input {}, must be unsigned 8 bit interger.", w)))
        .collect();

    println!("{}", bytes_to_string(&bytes[..], true));
}