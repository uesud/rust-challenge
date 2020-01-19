//! 与えられたIPv4アドレス(ドット形式)の範囲の値を列挙するサンプル

mod common;
use common::read_ipv4;

pub fn main() {
    if let Some(ipv4_first) = read_ipv4("first") {
        if let Some(ipv4_last) = read_ipv4("last") {
            for a in ipv4_first..=ipv4_last {
                println!("{}", a);
            }        
        }
    }
}