//! Chapter 3. 文字列と正規表現

use failure::Error;

/// 8bit数値列を16進文字列表記に変換する
pub fn bytes_to_string(bytes: &[u8], uppercase: bool) -> String {
    let mut txt = String::new();

    for b in bytes.iter() {
        txt.push_str(&format!("{:02x}", b));
    }

    if uppercase {
        txt.to_uppercase()
    } else {
        txt
    }
}

#[test]
fn test_bytes_to_string() {
    assert_eq!("baadf00d", bytes_to_string(&[0xba, 0xad, 0xf0, 0x0d], false));
    assert_eq!("BAADF00D", bytes_to_string(&[0xba, 0xad, 0xf0, 0x0d], true));

    assert_eq!("010203040506", bytes_to_string(&[1, 2, 3, 4, 5, 6], true));
}

/// 16進文字列表記から数値列に変換する
pub fn string_to_bytes(txt: &str) -> Result<Vec<u8>, Error> {
    
    let mut bytes = Vec::<u8>::new();
    for i in (0..(txt.len()+1)/2).map(|n| n * 2) {
        let end = if i == txt.len() - 1 { i + 1 } else { i + 2 };

        match u8::from_str_radix(&txt[i..end], 16) {
            Ok(b) => bytes.push(b),
            Err(_) => return Err(format_err!("Invalid input {}", &txt[i..end])),
        }
    }

    Ok(bytes)
}

#[test]
fn test_string_to_bytes() {
    assert_eq!(vec![0xBA, 0xAD, 0xF0, 0x0D], string_to_bytes(&"BAADF00D").unwrap())
}