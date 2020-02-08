//! 2次元配列を扱うサンプル

use rust_challenge::language_features::array2d::Array2d;

pub fn main() {
    let mut ma = Array2d::new(2,3);
    for i in 0..2 {
        for j in 0..3 {
            ma[(i,j)] = ((i+1) * 10 + (j+1)) as u8;
        }
    }

    for v in &ma {
        println!("{}", v);
    }

    println!("");

    ma.fill(255);
    for v in ma {
        println!("{}", v);
    }
}