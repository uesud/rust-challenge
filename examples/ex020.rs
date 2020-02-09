//! 任意個数の要素に対するコンテナ操作のサンプル

use rust_challenge::{contains_any, contains_all, contains_none};

pub fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    println!("contains_all: {}", contains_any!(v, &0, &3, &30));

    let a = [1, 2, 3, 4, 5, 6];
    println!("contains_all: {}", contains_all!(a, &1, &3, &5, &6));

    let s = [0, 1, 2, 3, 4, 5, 6, 7];
    let s1 = &s[1..7];
    println!("contains_none: {}", contains_none!(s1, &0, &6));
}