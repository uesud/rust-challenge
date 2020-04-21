/// 英字タイトルのキャピタライズのサンプル

mod common;
use common::read_string;
use rust_challenge::strings_regularexpressions::capitalize;

pub fn main() {
    let title = read_string("title");
    let capitalized = capitalize(&title);
    println!("{}", capitalized);
}