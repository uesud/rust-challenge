//! 与えられたIPv4アドレス(ドット形式)をIPV4に変換するサンプル

use rust_callenge::language_features::{Ipv4, ParseIpv4Error};

mod common;
use common::read_string;

pub fn main() {
    let ipv4_str = read_string("IPv4");

    match Ipv4::parse(&ipv4_str) {
        Ok(ipv4) => println!("{}", ipv4),
        Err(ParseIpv4Error::InvalidValue(s)) => println!("Invalid value: {}", s),
        Err(ParseIpv4Error::InvalidFormat) => println!("Invalid input"),
    }
}