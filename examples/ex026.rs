/// 区切り文字の連結のサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::join_strings;

pub fn main() {
    let input_strings = read_string("strings (comma separated)");
    let output_strings = join_strings(input_strings.split(',').collect(), ' ');
    println!("{}", output_strings);
}