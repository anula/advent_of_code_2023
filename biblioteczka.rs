#![allow(dead_code)]
// a^k % q
pub fn fast_pow_modulo(a: i64, k: i64, q: i64) -> i64 {
    if k == 1 {
        return a % q;
    }
    if k == 0 {
        return 1;
    }
    let half = fast_pow_modulo(a, k / 2, q) as i64;
    let mut res = (half * half) % q as i64;
    if k % 2 == 1 {
        res *= a as i64;
        res %= q as i64;
    }
    res
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

pub fn gcd(a: i64, b: i64) -> i64 {
    use std::cmp::{max, min};

    let mut big = max(a, b);
    let mut sml = min(a, b);

    while sml > 0 {
        let t = big;
        big = sml;
        sml = t % sml;
    }

    return big
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_pow_modulo() {
        let high_q = 1000000007;
        let small_q = 1021;
        assert_eq!(fast_pow_modulo(2, 1, high_q), 2);
        assert_eq!(fast_pow_modulo(2, 10, high_q), 1024);
        assert_eq!(fast_pow_modulo(5, 10, small_q), 781);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 5), 10);
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(13, 2), 26);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 5), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(13, 2), 1);
        assert_eq!(gcd(42, 28), 14);
        assert_eq!(gcd(13, 53), 1);
    }

}
