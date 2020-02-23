//! Chapter 3. 文字列と正規表現

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