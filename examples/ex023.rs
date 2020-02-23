/// 入力された数値列([u8])を16進文字列表記に変換するサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::bytes_to_string;

pub fn main() {
    let bytes: Vec::<u8> = read_string("8 bit numbers, separated by comma")
        .split(',')
        .collect::<Vec::<&str>>()
        .into_iter()
        .map(|b| 
            b.trim()
            .parse::<u8>()
            .expect("Invalid input {},  must be unsigned 8bit number."))
        .collect();

    println!("{}", bytes_to_string(&bytes[..], true));
}