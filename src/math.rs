//! Chapter 1. 数学の問題

use std;

/// 与えられた上限 `upper_limit` までの3または5で割り切れる正の整数の総和を求める。
pub fn sum_of_naturals_divisible_by_3_5(upper_limit: u32) -> u64 {
    // 3の倍数の総和と5の倍数の総和から、15の倍数の総和を引く。    
    sum_of_mutiple(3, upper_limit/3) + sum_of_mutiple(5, upper_limit/5) - sum_of_mutiple(15, upper_limit/15)
}

#[test]
fn test_sum_of_naturals_divisible_by_3_5() {
    assert_eq!(60, sum_of_naturals_divisible_by_3_5(15));
    assert_eq!(60, sum_of_naturals_divisible_by_3_5(16));
    assert_eq!(78, sum_of_naturals_divisible_by_3_5(18));
}

/// 整数 `a` に対して、`a * i, i = 1..n` の総和を求める。
fn sum_of_mutiple(a: u32, n: u32) -> u64 {
    (n * (n + 1) * a / 2).into()
}

#[test]
fn test_sum_of_multiple() {
    assert_eq!(18, sum_of_mutiple(3, 3));
}

/// 整数 `a` , `b` の最大公約数を求める
pub fn gcd(a: u32, b: u32) -> u32 {
    // ユークリッドの互除法で解く
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(9, gcd(9, 27));
    assert_eq!(9, gcd(27, 18));
    assert_eq!(1, gcd(26, 27));
}

/// 整数 `a` , `b` の最小公倍数を求める
pub fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

#[test]
fn test_ldm() {
    assert_eq!(15, lcm(3, 5));
}

/// 与えられた `n` より小さな素数のリストを返す
fn prime(n: u32) -> Vec<u32> {

    // エラトステネスの篩で素数の一覧を求める

    let limit = (n as f64).sqrt().floor() as u32;
    let mut v = (2..=n).rev().collect::<Vec<u32>>();
    let mut p = Vec::with_capacity(limit as usize);

    while let Some(x) = v.pop() {
        // xを素数リストに加える
        p.push(x);

        // xの倍数をvから除外する
        for i in (0..v.len()).rev() {
            if v[i] % x == 0 {
                v.remove(i);
                }
            }

        // 閾値まできたら探索を打ち切り、vの残りを素数リストに加える
        if x > limit {
            v.reverse();
            p.extend(v);
            break;    
        }
    }

    // n が素数の場合はpに含まれるので除外する
    if Some(&n) == p.last() {
        p.pop();
    }

    p
}

#[test]
fn test_prime() {
    assert_eq!(prime(1), []);
    assert_eq!(prime(2), []);
    assert_eq!(prime(3), [2]);
    assert_eq!(prime(11), [2,3,5,7]);
}

/// 与えられた `n` より小さな最大の素数を返す
pub fn largest_prime(n: u32) -> Option<u32> {
    prime(n).pop()
}

#[test]
fn test_largest_prime() {
    assert_eq!(largest_prime(1), None);
    assert_eq!(largest_prime(2), None);
    assert_eq!(largest_prime(3), Some(2));
    assert_eq!(largest_prime(3572), Some(3571));
}

/// 与えられた `n` より小さな素数からセクシー素数になる組のリストを返す
pub fn sexy_prime_pair(n: u32) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let p = prime(n);
    for x in p.iter() {
        if p.contains(&(x + 6)) {
            v.push((*x, x+6));
        }
    }

    v
}

#[test]
fn test_sexy_prime_pair() {
    assert_eq!(sexy_prime_pair(10), []);
    assert_eq!(sexy_prime_pair(20), [(5,11), (7,13), (11,17), (13,19)]);
}

/// 与えられた `n` の約数(自身は含まない)の総和を返す
fn sum_of_divisors(n: u32) -> u32 {
    let limit = n / 2;
    let mut s: u32 = 0;

    for i in 1..=limit {
        if n % i == 0 {
            s += i;
        }
    }

    s
}

#[test]
fn test_divisors() {
    assert_eq!(sum_of_divisors(7), 1);
    assert_eq!(sum_of_divisors(15), 1+3+5);
}

/// 与えられた上限 `upper_limit` までの過剰数のリストを返す
pub fn abundant_numbers(upper_limit: u32) -> Vec<u32> {
    let mut v = Vec::new();

    for i in 1..upper_limit {
        if sum_of_divisors(i) > i {
            v.push(i);
        }
    }

    v
}

#[test]
fn test_abundant_numbers() {
    assert_eq!(abundant_numbers(30), [12, 18, 20, 24]);
}

/// 与えられた上限 `upper_limit` までの友愛数の組のリストを返す
pub fn amicable_numbers(upper_limit: u32) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    for i in 1..upper_limit {
        let s = sum_of_divisors(i);
        if s > i && sum_of_divisors(s) == i {
            v.push((i, s));
        }
    }

    v
}

#[test]
fn test_amicable_numbers() {
    assert_eq!(amicable_numbers(3000), [(220,284), (1184,1210), (2620,2924)]);
}

/// 3桁のアームストロング数(ナルシシスト数)のリストを返す
pub fn narcissistic_number_3() -> Vec<u32> {
    // 3桁決め打ちで100から999まで探索する。
    let mut v = Vec::new();

    for i100 in 1..10u32 {
        let x100 = i100.pow(3);
        for i10 in 0..10u32 {
            let x10 = i10.pow(3);
            for i1 in 0..10u32 {
                let x1 = i1.pow(3);

                let n = i100 * 100 + i10 * 10 + i1;
                if n == x100 + x10 + x1  {
                    v.push(n);
                }
            }
        }
    }

    v
}

#[test]
fn test_narcissistic_number_3() {
    assert_eq!(narcissistic_number_3(), [153, 370, 371, 407]);
}

/// 与えられた `n` を素因数分解分解して (素数, 冪数) のリストを返す
pub fn prime_factorization(n: u32) -> Vec<(u32, u32)> {
    // n より小さい素数で順に冪数を求めていく。

    let mut v = Vec::new();

    let mut x = n;
    for p in prime(n) {
        let mut a = 0;        
        while x % p == 0 {            
            a += 1;
            x /= p;
        }

        if a > 0 {
            v.push((p, a));
        }
    }

    // n が素数か1なら自分自身を返す
    if n > 0 && v.len() == 0 {
        v.push((n, 1));
    }

    v
}

#[test]
fn test_prime_factorization() {
    assert_eq!(prime_factorization(0), []);    
    assert_eq!(prime_factorization(1), [(1,1)]);
    assert_eq!(prime_factorization(7), [(7,1)]);
    assert_eq!(prime_factorization(12), [(2,2), (3,1)]);
}

/// `b` をグレイコードにエンコードする
pub fn binary_to_gray(b: u8) -> u8 {
    b ^ (b >> 1)
}

#[test]
fn test_binary_to_gray() {
    assert_eq!(binary_to_gray(0b0101), 0b0111);
}

// `g` をグレイコードからデコードする
pub fn gray_to_binary(g: u8) -> u8 {
    let mut mask = g >> 1;
    let mut b = g;
    while mask != 0 {
        b ^= mask;
        mask >>= 1;
    }
    b
}

#[test]
fn test_gray_to_binary() {
    assert_eq!(gray_to_binary(0b0111), 0b0101);
}

/// 与えられた 'n' をローマ数字に変換する
pub fn num_to_roman(n: u32) -> String {

    let literal = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), 
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), 
        (1, "I")
    ];

    let mut num = n;
    let mut roman = String::new();

    for l in literal.iter() {
        while num >= l.0 {
            roman.push_str(l.1);
            num -= l.0;
        }
    }

    roman
}

#[test]
fn test_num_to_roman() {
    assert_eq!(num_to_roman(3999), "MMMCMXCIX");
}

/// 与えられた数 `n` のコラッツ数列の長さを返す。
/// 途中の計算過程は `counts` へキャッシュする。
/// `counts[n]` が、数 `n` のコラッツ数列長となる。
/// `counts` は任意長さが指定可能で、範囲を超える分は都度計算される。
fn count_collatz(n: u64, counts: &mut [u32]) -> u32 {
    if n < counts.len() as u64 {
        // 一度計算した値をキャッシュして再利用する
        if counts[n as usize] == 0 {
            if n == 1 {
                counts[n as usize] = 1;
            } else if n % 2 == 0 {
                counts[n as usize] = count_collatz(n / 2, counts) + 1;
            } else {
                counts[n as usize] = count_collatz(3 * n + 1, counts) + 1;
            }
        }
        counts[n as usize]
    } else {
        // キャッシュサイズより大きい数は都度計算する
        if n == 1 {
            1
        } else if n % 2 == 0 {
            count_collatz(n / 2, counts) + 1
        } else {
            count_collatz(3 * n + 1, counts) + 1
        }    
    }
}

#[test]
fn test_count_collatz() {
    let mut counts = vec![0; 14];
    assert_eq!(count_collatz(13, &mut counts), 10);
}

/// 1,000,000までの数のうち、(最長コラッツ数列になる数, その数列の長さ) の組を返す
pub fn max_len_of_collatz_1_000_000() -> (u32, u32) {
    let mut counts = vec![0; 1_000_001];

    let mut max_count = 0;
    let mut max_index = 0;

    for i in 1..1_000_001 {
        let count = count_collatz(i, &mut counts);
        if count > max_count {
            max_count = count;
            max_index = i;
        }
    }

    (max_index as u32, max_count)
}

#[test]
fn test_max_len_of_collatz_1_000_000() {
    assert_eq!(max_len_of_collatz_1_000_000(), (837799, 525));
}

/// (だいたい)小数点以下 `n` の精度で円周率を計算する (Gauss-Lagendre algorithm)
pub fn pi_gauss_legendre(n: u32) -> f64 {
    let mut a_0 = 1f64;
    let mut b_0 = 1f64 / 2f64.sqrt();
    let mut t_0 = 1f64 / 4f64;
    let mut p_0 = 1f64;

    for _ in 0..=(n as f64).log2().ceil() as u32 {
        let a_1 = (a_0 + b_0) / 2f64;
        let b_1 = (a_0 * b_0).sqrt();
        let t_1 = t_0 - p_0 * (a_0 - a_1) * (a_0 - a_1);
        let p_1 = 2f64 * p_0;

        a_0 = a_1;
        b_0 = b_1;
        t_0 = t_1;
        p_0 = p_1;
    }

    (a_0 + b_0) * (a_0 + b_0) / t_0 / 4f64
}

/// ISBN-10 パースエラー
#[derive(Debug, PartialEq)]
pub enum Isbn10ParseError {
    /// 不正な文字が含まれている
    InvalidCharacter(usize),
    /// ISBN-10 フォーマットエラー (10桁の数値列になっていない)
    InvalidLength,
    /// チェックディジットが一致しない
    InvalidNumber,
}

/// 与えられたISBN-10形式の文字列を数値列に変換する
pub fn parse_isbn10(isbn: &str) -> Result<Vec<u32>, Isbn10ParseError> {
    use std::str::from_utf8;

    let mut code = Vec::new();

    let isbnbytes = isbn.as_bytes();
    for pos in 0..isbnbytes.len() {
        match isbnbytes[pos] {
            b'0'..=b'9' => code.push(from_utf8(&isbnbytes[pos..=pos]).unwrap().parse().unwrap()),
            b'x' | b'X' => {
                if code.len() == 9 {
                    code.push(10);
                } else {
                    return Err(Isbn10ParseError::InvalidCharacter(pos));
                }
            },
            c => {
                if c != b'-' {
                    return Err(Isbn10ParseError::InvalidCharacter(pos));
                }
            },
        }
    }

    if code.len() != 10 {
        return Err(Isbn10ParseError::InvalidLength);
    }

    let mut check = 0;
    for n in 0..9 {
        check += code[n] * (10 - n as u32);
    }
    check = 11 - check % 11;
    if check == 11 {
        check = 0;
    }

    if check != code[9] {
        return Err(Isbn10ParseError::InvalidNumber);
    }
    
    Ok(code)
}

#[test]
fn test_parse_isbn10() {    
    assert_eq!(parse_isbn10("4-297-10559-4").unwrap(), [4,2,9,7,1,0,5,5,9,4]);
    assert_eq!(parse_isbn10("4-297-11559-x").unwrap(), [4,2,9,7,1,1,5,5,9,10]);
    assert_eq!(parse_isbn10("4-297-11559-X").unwrap(), [4,2,9,7,1,1,5,5,9,10]);
    assert_eq!(parse_isbn10("4-297-10559-3").unwrap_err(), Isbn10ParseError::InvalidNumber);
    assert_eq!(parse_isbn10("4-29X-10559-4").unwrap_err(), Isbn10ParseError::InvalidCharacter(4));
    assert_eq!(parse_isbn10("4-297-1055-4").unwrap_err(), Isbn10ParseError::InvalidLength);
}