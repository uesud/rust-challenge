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

    loop {
        match v.pop() {
            Some(x) => {
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
            },
            None => break,
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