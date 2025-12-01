/// Calculate the factorial of a number.
///
/// Returns `None` if the result would overflow u64.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::factorial;
///
/// assert_eq!(factorial(5), Some(120));
/// assert_eq!(factorial(0), Some(1));
/// assert_eq!(factorial(21), None); // Overflows u64
/// ```
pub fn factorial(n: u64) -> Option<u64> {
    if n > 20 {
        // 21! overflows u64
        return None;
    }

    let mut result = 1u64;
    for i in 2..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

/// Calculate the greatest common divisor (GCD) of two numbers using Euclidean algorithm.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::gcd;
///
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(100, 50), 50);
/// assert_eq!(gcd(17, 19), 1);
/// assert_eq!(gcd(0, 5), 5);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Check if a number is prime.
///
/// Returns `true` if the number is prime, `false` otherwise.
/// Note: 0 and 1 are not considered prime.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::is_prime;
///
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(17), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(1), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        _ => {
            let sqrt_n = (n as f64).sqrt() as u64;
            !(3..=sqrt_n).step_by(2).any(|i| n % i == 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_small_numbers() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
        assert_eq!(factorial(2), Some(2));
        assert_eq!(factorial(3), Some(6));
        assert_eq!(factorial(4), Some(24));
        assert_eq!(factorial(5), Some(120));
    }

    #[test]
    fn test_factorial_larger_numbers() {
        assert_eq!(factorial(10), Some(3_628_800));
        assert_eq!(factorial(15), Some(1_307_674_368_000));
        assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
    }

    #[test]
    fn test_factorial_overflow() {
        assert_eq!(factorial(21), None);
        assert_eq!(factorial(25), None);
        assert_eq!(factorial(100), None);
    }

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6); // Commutative
        assert_eq!(gcd(100, 50), 50);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(13, 7), 1);
    }

    #[test]
    fn test_gcd_same_numbers() {
        assert_eq!(gcd(42, 42), 42);
        assert_eq!(gcd(1, 1), 1);
    }

    #[test]
    fn test_gcd_with_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_large_numbers() {
        assert_eq!(gcd(1_000_000, 500_000), 500_000);
        assert_eq!(gcd(123_456_789, 987_654_321), 9);
    }

    #[test]
    fn test_is_prime_small_primes() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
    }

    #[test]
    fn test_is_prime_small_composites() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
    }

    #[test]
    fn test_is_prime_edge_cases() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_is_prime_larger_primes() {
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(23), true);
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(101), true);
        assert_eq!(is_prime(1009), true);
    }

    #[test]
    fn test_is_prime_larger_composites() {
        assert_eq!(is_prime(100), false);
        assert_eq!(is_prime(121), false); // 11 * 11
        assert_eq!(is_prime(1000), false);
    }
}
