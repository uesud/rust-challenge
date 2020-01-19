//! Chapter 2. 言語機能

use std::iter::Step;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ipv4 {
    value: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseIpv4Error {
    // 不正な値
    InvalidValue(String),
    // フォーマットエラー
    InvalidFormat,
}

impl Ipv4 {
    /// 8bitの4組(a1.a2.a3.a4)に相当する Ipv4 を返す
    pub fn from_dot_decimals(a1: u8, a2: u8, a3: u8, a4: u8) -> Self {
        Ipv4 { value: (a1 as u32)  << 24 | (a2 as u32) << 16 | (a3 as u32) << 8 | a4 as u32 }
    }

    /// ドットアドレス形式の文字列に相当する Ipv4 を返す
    pub fn parse(ipv4: &str) -> Result<Self, ParseIpv4Error> {
        let v: Vec<&str> = ipv4.split('.').collect();
        if v.len() == 4 {
            Ok(Ipv4::from_dot_decimals(
                v[0].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[0].to_owned()))?,
                v[1].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[1].to_owned()))?,
                v[2].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[2].to_owned()))?,
                v[3].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[3].to_owned()))?
            ))
        } else {
            Err(ParseIpv4Error::InvalidFormat)
        }
    }
}

impl fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>03}.{:>03}.{:>03}.{:>03}", 
            (self.value >> 24) & 0x00ff,
            (self.value >> 16) & 0x00ff,
            (self.value >> 8) & 0x00ff,
            self.value & 0x00ff
        )
    }
}

impl Step for Ipv4 {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end.value <= start.value {
            Some((end.value - start.value) as usize)
        } else {
            None
        }
    }

    fn replace_one(&mut self) -> Self {
        Ipv4 { value: 1 }        
    }

    fn replace_zero(&mut self) -> Self {
        Ipv4 { value: 0 }
    }
    
    fn add_one(&self) -> Self {
        Ipv4 { value: self.value + 1 }
    }

    fn sub_one(&self) -> Self {
        Ipv4 { value: self.value - 1 }
    }

    fn add_usize(&self, n: usize) -> Option<Self> {
        if self.value as usize + n <= std::u32::MAX as usize {
            Some(Ipv4 { value: self.value + n as u32 })
        } else {
            None
        }
    }
}

#[test]
fn test_ipv4_parse() {
    assert_eq!(Ipv4::parse("1.2.3.4").unwrap(), Ipv4::from_dot_decimals(1,2,3,4));
}

#[test]
fn test_ipv4_rangeiter() {
    let a1 = Ipv4::from_dot_decimals(1, 2, 3, 4);
    let a2 = Ipv4::from_dot_decimals(1, 2, 3, 6);

    let mut v = Vec::new();
    for a in a1..=a2 {
        v.push(a);
    }

    assert_eq!(v,
        [
            Ipv4::from_dot_decimals(1, 2, 3, 4), 
            Ipv4::from_dot_decimals(1, 2, 3, 5),
            Ipv4::from_dot_decimals(1, 2, 3, 6)
        ]);
}