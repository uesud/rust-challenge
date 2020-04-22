/// 文字列をトークンに分割するサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::split_string;

pub fn main() {
    let input_strings = read_string("strings (comma separated)");
    let output_tokens = split_string(&input_strings, vec![',']);
    for token in output_tokens {
        println!("{}", token);
    }
}