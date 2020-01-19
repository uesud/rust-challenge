//! 与えられたIPv4アドレス(ドット形式)をIPV4に変換するサンプル

mod common;
use common::read_ipv4;

pub fn main() {
    if let Some(ipv4) = read_ipv4("IPv4") {
        println!("{}", ipv4)
    }
}