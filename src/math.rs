//! Chapter 1. 数学の問題

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

/// 整数 a, b の最大公約数を求める
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