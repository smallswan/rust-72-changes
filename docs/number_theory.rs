//! # 数论模块
//!
//! 对应第五十七章，涵盖 GCD、素数、模运算、欧拉函数等。

/// 辗转相除法求最大公约数
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// 最小公倍数（先除后乘避免溢出）
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

/// 扩展欧几里得算法：返回 (gcd, x, y) 使得 a*x + b*y = gcd
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

/// 模逆元：求 a^{-1} mod m，当 gcd(a,m)=1 时存在
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = ext_gcd(a, m);
    if g != 1 {
        None
    } else {
        Some(((x % m) + m) % m)
    }
}

/// 快速幂（模幂运算）：计算 a^b mod m
pub fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u128;
    let base = (base as u128) % (modulus as u128);
    let modulus = modulus as u128;
    let mut base = base;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }
    result as u64
}

/// 试除法素数判定
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Miller-Rabin 素性测试（确定性，适用于 u64 范围）
pub fn miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    // 分解 n-1 = d * 2^r
    let mut d = n - 1;
    let mut r = 0u32;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    // 确定性测试基（适用于 u64）
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    'witness: for &a in &witnesses {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            // 使用 u128 避免溢出
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// 埃拉托斯特尼筛法：返回 n 以内所有素数
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut is_prime_arr = vec![true; n + 1];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime_arr[i] {
            let mut j = i * i;
            while j <= n {
                is_prime_arr[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime_arr
        .iter()
        .enumerate()
        .filter(|&(_, &p)| p)
        .map(|(idx, _)| idx)
        .collect()
}

/// 欧拉函数 φ(n)：1 到 n-1 中与 n 互质的正整数个数
pub fn euler_totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(100, 0), 100);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(7, 3), 21);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 5, 7), 5);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(13));
        assert!(!is_prime(15));
        assert!(is_prime(997));
    }

    #[test]
    fn test_miller_rabin() {
        assert!(miller_rabin(997));
        assert!(miller_rabin(7919));
        assert!(!miller_rabin(1001));
        assert!(!miller_rabin(u64::MAX)); // 2^64-1 不是素数
    }

    #[test]
    fn test_sieve() {
        let primes = sieve_of_eratosthenes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(6), 2);
        assert_eq!(euler_totient(10), 4);
        assert_eq!(euler_totient(12), 4);
    }

    #[test]
    fn test_mod_inverse() {
        let inv = mod_inverse(3, 11).unwrap();
        assert_eq!((3 * inv) % 11, 1);
        assert!(mod_inverse(4, 8).is_none()); // gcd(4,8)=4≠1
    }
}
