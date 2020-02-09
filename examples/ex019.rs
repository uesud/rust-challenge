//! 任意個数の要素をコンテナに追加するサンプル

use rust_challenge::push_back;

pub fn main() {
    let mut v = vec![1,2,3];
    push_back!(v, 4, 5);
    println!("v = {:?}", v);
}