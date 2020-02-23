/// 数値列([u8])と16進文字列表記を相互に変換するサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::{string_to_bytes};

pub fn main() {
    let txt = read_string("n");
    println!("{:?}", string_to_bytes(&txt));
}