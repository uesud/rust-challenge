//! 与えられた文字列が適切なISBN-10かチェックするサンプル

use rust_challenge::math::{parse_isbn10, Isbn10ParseError};

mod common;
use common::read_string;

pub fn main() {
    let text = read_string("n");
    match parse_isbn10(&text) {
        Ok(_) => println!("Valid"),
        Err(Isbn10ParseError::InvalidNumber) => println!("Invalid"),
        Err(Isbn10ParseError::InvalidCharacter(pos)) => {
            let mut annotation = String::new();
            (0..pos).for_each(|_| annotation.push(' '));
            annotation.push('^');

            println!("Invalid\n{}\n{}", text, annotation);
        },
        Err(Isbn10ParseError::InvalidLength) => println!("Invalid"),
    }
}