//! 3桁のアームストロング数を出力するサンプル

use rust_challenge::math::narcissistic_number_3;

pub fn main() {
    for x in narcissistic_number_3() {
        println!("{}", x);
    }
}