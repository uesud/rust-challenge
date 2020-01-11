//! 与えられた整数を素因数分解して表示するサンプル

use rust_callenge::math::prime_factorization;

mod common;
use common::read_u32;

pub fn main() {
    let n = read_u32("n");

    let mut is_first = true;
    let mut txt = String::new();
    for (p, a) in prime_factorization(n) {
        if is_first {
            is_first = false;
        } else {
            txt.push_str(" * ");
        }

        if a > 1 {
            txt.push_str(&format!("{}^{}", p, a));
        } else {
            txt.push_str(&format!("{}", p));
        }
    }
    println!("{}", txt);
}